//! A "step" in a series of steps that completes a process. The step should be recoverable.
//! Meaning that if a step fails, the process can be restarted and all previously completed steps
//! will be skipped.

pub mod event;
mod executor;
pub mod msg_handler;
mod sequence;

use async_trait::async_trait;
use std::{
    collections::HashMap,
    fmt::Display,
    marker::{Send, Sync},
};

pub use executor::{ExecutorProperties, StepExecutor};
pub use sequence::{NextSequenceStep, StepSequence};

/// StepItem wrapped in a Box that implements Sync + Send to send the StepItem between threads for
/// execution and message processing.
pub type BoxedStepItem = Box<dyn StepItem + Sync + Send>;

/// A Step can be declared as a single step, or a sequence of steps. A sequence is useful to define
/// on order of dependencies, where multiple individual steps can be completed at once.
pub enum Step {
    /// A single step item.
    Step(BoxedStepItem),
    /// A sequence of many step items.
    Sequence(StepSequence),
    Parallel(ParallelSteps),
}

impl Step {
    fn get_id(&self) -> StepId {
        match self {
            Step::Step(step) => step.get_id(),
            Step::Sequence(seq) => seq.get_id(),
            Step::Parallel(par) => par.get_id(),
        }
    }

    /// Too lazy should return struct
    async fn should_run(&mut self, steps: Steps) -> Vec<(StepId, ShouldRunResult)> {
        let mut steps = steps;
        match self {
            Step::Step(step) => {
                let should_run = step.should_run().await;
                vec![(step.get_id(), should_run)]
            }
            Step::Sequence(seq) => {
                let step_ids = seq.get_next();
                let mut results = Vec::new();
                for step_id in step_ids {
                    let step = steps.take(&step_id);
                    let should_run = match step {
                        Some(mut step) => {
                            let res = step.should_run(steps).await;
                            steps.set(step);
                            res
                        }
                        None => {
                            unimplemented!();
                        }
                    };
                    results.extend(should_run);
                }
                results
            }
            Step::Parallel(par) => {
                let step_ids = par.steps;
                let mut results = Vec::new();
                for step_id in step_ids {
                    let step = steps.take(&step_id);
                    let should_run = match step {
                        Some(step) => {
                            let res = step.should_run(steps).await;
                            steps.set(step);
                            res
                        }
                        None => {
                            unimplemented!();
                        }
                    };
                    results.extend(should_run);
                }
                results
            }
        }
    }
}

#[async_trait]
pub trait StepItem {
    /// An asynchronous function that will be called to determine if this step should be executed.
    async fn should_run(self: Box<Self>) -> ShouldRunResult;
    /// The body of work to be completed by this step.
    async fn execute(self: Box<Self>) -> Result<String, String>;
}

#[derive(Clone)]
/// Contains the details of a step. Used to pass information about progress to the message handler.
pub struct StepDetails {
    pub title: String,
}

pub enum ShouldRunResult {
    /// The step should be run.
    Ok,
    /// The step has already completed and skipped.
    Skip,
    /// A hard error that should interrupt and exit.
    Error(StepProcessError),
}

pub struct StepProcessError {
    pub message: String,
}

impl StepProcessError {
    pub fn new(message: &str) -> StepProcessError {
        StepProcessError {
            message: message.to_string(),
        }
    }
}

impl Display for StepProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A collection of steps to run in parallel.
pub struct ParallelSteps {
    step_id: StepId,
    pub steps: Vec<StepId>,
}

impl ParallelSteps {
    pub fn new(steps: Vec<StepId>) -> ParallelSteps {
        ParallelSteps {
            step_id: StepId::new_random(),
            steps,
        }
    }

    pub fn get_id(&self) -> StepId {
        self.step_id
    }
}

#[derive(Clone, Eq, Hash)]
pub struct StepId {
    pub id: uuid::Uuid,
}

impl StepId {
    /// Generates a random id using UUID v4.
    pub fn new_random() -> StepId {
        StepId {
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl PartialEq for StepId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait StepWithId {
    fn get_id(&self) -> StepId;
}

pub struct Steps {
    inner: HashMap<StepId, Step>,
}

impl Steps {
    pub fn new() -> Steps {
        Steps {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, step: Step) {
        let id = step.get_id();
        self.inner.insert(id, step);
    }

    pub fn set(&mut self, step: Step) {
        let id = step.get_id();
        self.inner.insert(id, step);
    }

    pub fn take(&self, id: &StepId) -> Option<Step> {
        self.inner.remove(id)
    }
}
