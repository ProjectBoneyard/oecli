//! Uses ProgressBars per sequence to inform the user the progress of the current command ran.

use crate::log::LogLevel;
use crate::step::event::{EventData, NewSequenceEvent};
use crate::step::msg_handler::EventHandler;
use crate::step::StepDetails;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;

/// Msg implementation that will use [indicatif](indicatif) and create various progress bars.
/// Progress bars are used to inform the user of the progress.
pub struct ProgressBarsEventHandler {
    pub msg_level: LogLevel,
    pub overall_progress: MultiProgress,
    pub progress_bars: HashMap<String, ProgressBar>,
}

impl ProgressBarsEventHandler {
    pub fn new(msg_level: LogLevel) -> ProgressBarsEventHandler {
        ProgressBarsEventHandler {
            msg_level,
            overall_progress: MultiProgress::new(),
            progress_bars: HashMap::new(),
        }
    }
}

impl EventHandler for ProgressBarsEventHandler {
    /// Any time a new sequence is started we want to ensure that a new progress bar is crated to
    /// track the progress of steps within that sequence.
    fn sequence_start(&mut self, details: NewSequenceEvent) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                if self.progress_bars.contains_key(&details.sequence_name) {
                    return;
                }
                let pb = self
                    .overall_progress
                    .add(ProgressBar::new(details.length as u64));
                let sty =
                    ProgressStyle::with_template("[{pos:>4}/{len:4}] {bar:20} {msg}").unwrap();
                pb.set_style(sty);
                self.progress_bars.insert(details.sequence_name, pb);
            }
        }
    }

    /// Once a sequence has completed we want to remove the progress bar associated with it.
    fn sequence_end(&mut self, event_details: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                let sequence_name = event_details.sequence_name.to_owned();
                let pb = self.progress_bars.get(&sequence_name);
                if let Some(pb) = pb {
                    pb.finish();
                }
            }
        }
    }

    /// Once a step starts we inform the user what step is starting and tick the progress bar to
    /// initiate rendering.
    fn step_start(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                let pb = self.progress_bars.get(&data.sequence_name);
                if let Some(pb) = pb {
                    pb.set_message(format!("{}", step.title));
                    pb.tick();
                }
            }
        }
    }

    /// Anytime we bump the progress of a step we bump the associated progress bar.
    fn step_skipped(&mut self, step: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                let pb = self.progress_bars.get(&data.sequence_name);
                if let Some(pb) = pb {
                    pb.set_message(format!(
                        "{}: {} {}",
                        step.title,
                        data.msg.to_owned(),
                        pb.position()
                    ));
                    pb.inc(1);
                }
            }
        }
    }

    /// Once a step is completed successfully, we bump the progress of the progress bar.
    fn step_end(&mut self, _: StepDetails, data: &EventData) {
        match &self.msg_level {
            LogLevel::Silent => {}
            _ => {
                let pb = self.progress_bars.get(&data.sequence_name);
                if let Some(pb) = pb {
                    pb.inc(1)
                }
            }
        }
    }

    /// Error handling is handled at the top level.
    fn step_error(&mut self, _step: StepDetails, _data: &EventData) {}
}
