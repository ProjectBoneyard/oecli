//! Optionally print all event messages to the console.

use crate::log::LogLevel;
use crate::step::event::{EventData, NewSequenceEvent};
use crate::step::msg_handler::EventHandler;
use crate::step::StepDetails;

/// Msg implementation that will use [println!()]() to inform the user of the progress.
pub struct ConsoleLogEventHandler {
    pub msg_level: LogLevel,
}

impl ConsoleLogEventHandler {
    #[allow(dead_code)]
    pub fn new(msg_level: LogLevel) -> ConsoleLogEventHandler {
        ConsoleLogEventHandler { msg_level }
    }
}

impl EventHandler for ConsoleLogEventHandler {
    fn sequence_start(&mut self, details: NewSequenceEvent) {}
    fn sequence_end(&mut self, details: &EventData) {}

    fn step_start(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            LogLevel::Verbose => {
                println!("{}\n{}\n{}", &step.title, &step.description, data.msg);
            }
            LogLevel::Info => {
                println!("Begin: {}\n{}", &step.title, &step.description);
            }
            _ => {}
        }
    }

    fn step_end(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            LogLevel::Verbose => {
                println!("{} Completed\n{}", &step.title, data.msg);
            }
            LogLevel::Info => {
                println!("End: {}", &step.title);
            }
            _ => {}
        }
    }

    fn step_skipped(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            LogLevel::Verbose => {
                println!("{}: {}", &step.title, &data.msg);
            }
            LogLevel::Info => {
                println!("progress: {}: {}", &step.title, &data.msg);
            }
            _ => {}
        }
    }

    fn step_error(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                println!("Error: {}\n{}", &step.title, &data.msg);
            }
        }
    }
}
