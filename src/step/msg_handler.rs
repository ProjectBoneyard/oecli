//! Handler to be implemented that the [Executor](crate::step::StepExecutor) will use to publish
//! lifecycle events to.

mod console;
mod progress_bars;

use crate::step::event::{EventData, NewSequenceEvent, StepEvent};
use crate::step::StepDetails;
pub use console::ConsoleLogEventHandler;
pub use progress_bars::ProgressBarsEventHandler;

/// The Steps are executed asynchronously, and on a multi-threaded machine can run on other thread.
/// The executor supports a Message handler that is shared behind a Mutex so it can be safely
/// shared between threads. Each implementation of a Msg can listen to different events and handle
/// them accordingly.
///
/// This should be renamed to event. Event Handler makes more sense.
pub trait EventHandler {
    fn handle_event(&mut self, step: Option<StepDetails>, event: StepEvent<'_>) {
        if let StepEvent::NewSequence(sequence_details) = event {
            self.sequence_start(sequence_details);
            return;
        }
        if let StepEvent::EndSequence(sequence_details) = event {
            self.sequence_end(sequence_details);
            return;
        }
        let step = step.unwrap();
        match event {
            StepEvent::Start(details) => {
                self.step_start(step, details);
            }
            StepEvent::End(details) => {
                self.step_end(step, details);
            }
            StepEvent::Skip(details) => {
                self.step_skipped(step, details);
            }
            StepEvent::Error(details) => {
                self.step_error(step, details);
            }
            _ => {}
        }
    }

    fn sequence_start(&mut self, details: NewSequenceEvent);
    fn sequence_end(&mut self, details: &EventData);
    fn step_start(&mut self, step: StepDetails, details: &EventData);
    fn step_skipped(&mut self, step: StepDetails, details: &EventData);
    fn step_end(&mut self, step: StepDetails, details: &EventData);
    fn step_error(&mut self, step: StepDetails, details: &EventData);
}
