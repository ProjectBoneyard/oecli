//! A series of steps that will run in series.

use crate::step::{BoxedStepItem, Step, StepId};
use std::collections::VecDeque;

/// A series of steps that will run in series. Each individual step can itself be either a
/// [StepItem]() or a StepSequence.
///
/// Example:
///
/// ```
/// let step1 = StepItem::new("step1");
/// let step2 = StepItem::new("step2");
/// let step3 = StepItem::new("step3");
/// let step4 = StepItem::new("step4");
/// let seq = StepSequence::new("Unique Sequence Name", "Detailed Description")
///    .set_steps(vec![Step::Step(Box::new(step1))])
///    .then_run(Step::Step(Box::new(step2)));
///    .then_parallel(vec![Step::Step(Box::new(step3)), Step::Step(Box::new(step4))]);
///
/// assert_eq!(seq.title, "Unique Sequence Name");
/// assert_eq!(seq.description, "Detailed Description");
/// assert_eq!(seq.num_step(), 4);
/// assert_eq!(seq.has_next(), true);
/// ```
pub struct StepSequence {
    /// Current iteration in the sequence.
    cur: usize,
    /// A description of what is expected to happen within this sequence.
    pub description: String,
    pub step_id: StepId,
    pub steps: Vec<Vec<StepId>>,
    /// A unique name for the step sequence.
    pub title: String,
}

impl StepSequence {
    pub fn new(title: &str, description: &str) -> StepSequence {
        StepSequence {
            cur: 0,
            description: description.to_owned(),
            step_id: StepId::new_random(),
            steps: Vec::new(),
            title: title.to_owned(),
        }
    }

    /// Returns the step id
    pub fn get_id(&self) -> StepId {
        self.step_id
    }

    pub fn get_next(&mut self) -> Vec<StepId> {
        let step_ids = self.steps[self.cur];
        self.cur = self.cur + 1;
        step_ids
    }

    /// Checks if there are any remaining steps to be processed in the step queue.
    pub fn has_next(&self) -> bool {
        self.cur < self.steps.len()
    }

    /// Defines a number of steps to be run in parallel.
    pub fn set_steps(mut self, steps: Vec<StepId>) -> StepSequence {
        self.steps = vec![steps];
        self
    }

    /// Add a step to run at the end of the queue.
    pub fn then_run(mut self, step: StepId) -> StepSequence {
        self.steps.push(vec![step]);
        self
    }

    /// Add several steps to run in parallel to the end of the queue.
    pub fn then_run_parallel(mut self, steps: Vec<StepId>) -> StepSequence {
        self.steps.push(steps);
        self
    }
}
