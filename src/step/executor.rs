//! The Step Executor is responsible for processing [Steps](crate::step::Step) into
//! [StepItems](crate::step::StepItem) and executing them.

mod properties;

use crate::step::event::{EventData, NewSequenceEvent, StepEvent};
use crate::step::msg_handler::EventHandler;
use crate::step::{BoxedStepItem, ShouldRunResult, Step};
use futures::future::join_all;
use std::collections::VecDeque;
use std::fmt::Display;
use std::marker::{Send, Sync};
use std::sync::Arc;
use tokio::sync::Mutex;

pub use properties::ExecutorProperties;

/// Processor for taking a set of steps and executing them in order as described by the Executor Properties.
pub struct StepExecutor<TMH: EventHandler> {
    event_handler: Arc<Mutex<TMH>>,
    pub steps: Vec<Vec<Step>>,
}

impl<TMH: EventHandler + Sync + Send + 'static> StepExecutor<TMH> {
    pub fn new(event_handler: TMH) -> StepExecutor<TMH> {
        StepExecutor {
            event_handler: Arc::new(Mutex::new(event_handler)),
            steps: Vec::new(),
        }
    }

    /// From [ExecutorProperties]() sets up the steps to be executed.
    pub fn build_steps(mut self, properties: ExecutorProperties) -> StepExecutor<TMH> {
        self.steps = properties.get_steps();
        self
    }

    fn count_steps(self) -> Vec<Vec<Step>> {
        self.steps
    }

    /// Main processor function. Will push the [StepItems]() into a queue and process each step in the
    /// queue. As [StepSequences]() are come across, will add the next StepItem to the queue.
    ///
    /// Steps will be attempted to be executed asynchronously, either on one thread or many.
    pub async fn run(self) -> Result<(), String> {
        let all_steps = self.steps;
        // Publish the event for the main sequence.
        // Should move into own function
        {
            let num_steps = all_steps.iter().fold(0, |acc, step| acc + step.len());
            let mut event_handler = self.event_handler.lock().await;
            event_handler.handle_event(
                None,
                StepEvent::NewSequence(NewSequenceEvent::new(num_steps, "main")),
            );
        }

        // Populate initial queue
        // Move into it's own function
        let mut queue = VecDeque::new();
        for steps in all_steps {
            queue.push_back(steps);
        }

        // loop queue until empty
        // Move into it's own function
        loop {
            let mut join_handles = Vec::new();

            let parallel_steps = queue.pop_front();
            if let Some(parallel_steps) = parallel_steps {
                for step in parallel_steps {
                    // Top level StepItems are apart of the "main step sequence"
                    let mut sequence_name = "main".to_string();
                    if let Step::Sequence(seq) = &step {
                        sequence_name = seq.title.clone();
                    }
                    let event_handler = self.event_handler.clone();
                    let handle = tokio::spawn(async move {
                        process_step(step, event_handler, &sequence_name).await
                    });
                    join_handles.push(handle);
                }
            }

            let results = join_all(join_handles).await;
            let results = results
                .into_iter()
                .map(|r| r.unwrap())
                .collect::<Vec<Result<Option<Step>, String>>>();
            for result in &results {
                if let Err(e) = result {
                    return Err(e.to_string());
                }
            }
            let results = results
                .into_iter()
                .filter_map(|r| r.unwrap())
                .collect::<Vec<_>>();

            // We complete depth first, if there are any remaining StepItems to be processed we add
            // them to the queue.
            if !results.is_empty() {
                queue.push_front(results);
            }

            // Once the queue is empty, we are done.
            if queue.is_empty() {
                break;
            }
        }
        Ok(())
    }
}

/// Process a Step, for a [StepItem](), manages the standard lifecycle. For a [StepSequence](),
/// will attempt to grab the next "recursive" StepItem from that sequence.
async fn process_step<TM: EventHandler>(
    step: Step,
    event_handler: Arc<Mutex<TM>>,
    sequence_name: &str,
) -> Result<Option<Step>, String> {
    match step {
        Step::Step(step_item) => {
            let step_result =
                execute_step_item(step_item, event_handler, sequence_name.to_owned()).await;
            if step_result.is_err() {
                return Err(step_result.err().unwrap());
            }
        }
        Step::Sequence(mut sequence) => {
            let next_steps = sequence.get_next_sequence_steps();

            if !next_steps.is_empty() {
                // For each step to be processed we notify the message handler that we are
                // processing a new sequence.
                for next_seq_step in &next_steps {
                    let next_sequence_name = next_seq_step.sequence_name.clone();
                    {
                        let num_steps = sequence.num_steps();
                        let mut event_handler = event_handler.lock().await;
                        event_handler.handle_event(
                            None,
                            StepEvent::NewSequence(NewSequenceEvent::new(
                                num_steps + 1, // Need to account for popping of the next step
                                &next_sequence_name,
                            )),
                        );
                    }
                }

                let mut queue = Vec::new();
                for next_seq_step in next_steps {
                    let event_handler = event_handler.clone();
                    let next_sequence_name = next_seq_step.sequence_name.clone();
                    queue.push(execute_step_item(
                        next_seq_step.step,
                        event_handler,
                        next_sequence_name,
                    ));
                }
                let results = join_all(queue).await;
                for result in results {
                    if result.is_err() {
                        return Err(result.err().unwrap().to_string());
                    }
                }
            } else {
                // No more steps to process, so we are done. Notify the message handler that a
                // sequence has finished.
                let mut event_handler = event_handler.lock().await;
                event_handler.handle_event(
                    None,
                    StepEvent::EndSequence(&EventData::new(&sequence.title, sequence_name)),
                );
            }

            // If there are any remaining StepItems to be processed, we return them to be added to
            // the queue.
            if sequence.has_next() {
                return Ok(Some(Step::Sequence(sequence)));
            }
        }
    }

    // Once a StepItem is processed there are no longer any remaining StepItems to be processed.
    Ok(None)
}

/// Logic for managing a [StepItem]() through it's lifecycle. Verifies that the step should be
/// ran if it has, sill skip. Otherwise will run the main body of the StepItem.
async fn execute_step_item<TM: EventHandler>(
    step_item: BoxedStepItem,
    event_handler: Arc<Mutex<TM>>,
    sequence_name: String,
) -> Result<(), String> {
    let mut event_handler = event_handler.lock().await;
    let step_details = step_item.get_step_details();

    event_handler.handle_event(
        Some(step_details.clone()),
        StepEvent::Start(&EventData::new("", &sequence_name)),
    );

    let should_run = step_item.should_run().await;
    match should_run {
        ShouldRunResult::Ok => {
            // Should send the name of the sequence to the message handler so it can mark the progress
            // bar as complete
            let result = step_item.execute().await;
            match result {
                Ok(msg) => {
                    event_handler.handle_event(
                        Some(step_details.clone()),
                        StepEvent::End(&EventData::new(&msg, &sequence_name)),
                    );
                    return Ok(());
                }
                Err(msg) => {
                    event_handler.handle_event(
                        Some(step_details.clone()),
                        StepEvent::Error(&EventData::new(&msg, &sequence_name)),
                    );
                    return Err(msg);
                }
            }
        }
        ShouldRunResult::Skip => {
            event_handler.handle_event(
                Some(step_details.clone()),
                StepEvent::Skip(&EventData::new(
                    "Skipped. Already completed.",
                    &sequence_name,
                )),
            );
            return Ok(());
        }
        ShouldRunResult::Error(e) => {
            event_handler.handle_event(
                None,
                StepEvent::Error(&EventData::new(
                    &format!("Unexpected error processing step.\n{}", e.to_string()),
                    &sequence_name,
                )),
            );
            return Err(e);
        }
    }
}

// New executor

// struct that contains the steps to be executed
//
// struct that wraps the step to be executed. Contains exists as a mini state machine. Shows the
// status. Not Started, Should Run, Skipped, Error, Run, Completed.
//
// the executor will push all the steps into a queue and tokio spawn them all. The executor will
// join them all before collecting next set of steps. The executor will also notify the message
// handler of the status of the steps.

/// Processor for taking a set of steps and executing them in order as described by the Executor Properties.
pub struct StepExecutor2<TMH: EventHandler> {
    event_handler: Arc<Mutex<TMH>>,
    pub steps: Vec<Vec<Step>>,
}

impl<TMH: EventHandler + Sync + Send + 'static> StepExecutor2<TMH> {
    /// From [ExecutorProperties]() sets up the steps to be executed.
    pub fn from_executor_properties(
        properties: ExecutorProperties,
        event_handler: TMH,
    ) -> StepExecutor2<TMH> {
        StepExecutor2 {
            event_handler: Arc::new(Mutex::new(event_handler)),
            steps: properties.get_steps(),
        }
    }
}

impl<TMH: EventHandler + Sync + Send + 'static> StepExecutor2<TMH> {
    /// Main processor function. Will push the [StepItems]() into a queue and process each step in the
    /// queue. As [StepSequences]() are come across, will add the next StepItem to the queue.
    ///
    /// Steps will be attempted to be executed asynchronously, either on one thread or many.
    pub async fn run(self) -> Result<(), String> {
        // Create a queue of steps to be processed.
    }
}

enum StepStatus {
    NotStarted,
    ShouldRun,
    Skipped,
    Error,
    Completed,
}

impl Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepStatus::NotStarted => write!(f, "Not Started"),
            StepStatus::ShouldRun => write!(f, "Should Run"),
            StepStatus::Skipped => write!(f, "Skipped"),
            StepStatus::Error => write!(f, "Error"),
            StepStatus::Completed => write!(f, "Completed"),
        }
    }
}

struct StepState {
    step: BoxedStepItem,
    status: StepStatus,
}

impl StepState {
    fn new(step: BoxedStepItem) -> StepState {
        StepState {
            step,
            status: StepStatus::NotStarted,
        }
    }
}

impl StepState {
    async fn run(mut self) -> StepState {
        match self.status {
            StepStatus::NotStarted => {
                let result = self.step.should_run().await;
                match result {
                    ShouldRunResult::Ok => {
                        self.status = StepStatus::ShouldRun;
                    }
                    ShouldRunResult::Skip => {
                        self.status = StepStatus::Skipped;
                    }
                    ShouldRunResult::Error(e) => {
                        self.status = StepStatus::Error;
                    }
                }
                self.status = StepStatus::ShouldRun;
            }
            StepStatus::ShouldRun => {
                self.status = StepStatus::Completed;
            }
            _ => {}
        }
        self
    }
}
