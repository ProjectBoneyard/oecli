//!  ExecutorProperties defines the behaviors of the Step Executor.

use crate::step::Step;

/// Describes a way to define a series of steps to be executed and in which order or which can be
/// one asynchronously. ExecutorProperties are provided in the CommandExecutor and used to
/// construct the steps and execute them.
pub struct ExecutorProperties {
    steps: Vec<Vec<Step>>,
}

impl ExecutorProperties {
    pub fn new() -> ExecutorProperties {
        ExecutorProperties { steps: Vec::new() }
    }

    /// Set the step to be ran.
    #[allow(dead_code)]
    pub fn run(mut self, step: Step) -> ExecutorProperties {
        self.steps = vec![vec![step]];
        self
    }

    /// Set several steps to run in parallel.
    #[allow(dead_code)]
    pub fn run_parallel(mut self, steps: Vec<Step>) -> ExecutorProperties {
        self.steps = vec![steps];
        self
    }

    /// Will add a step to run after the previous step(s) were completed.
    #[allow(dead_code)]
    pub fn then_run(mut self, step: Step) -> ExecutorProperties {
        self.steps.push(vec![step]);
        self
    }

    /// Will add several steps to run after the previous step(s) were
    /// completed.
    #[allow(dead_code)]
    pub fn then_run_parallel(mut self, steps: Vec<Step>) -> ExecutorProperties {
        self.steps.push(steps);
        self
    }

    /// Consumes the ExecutorProperties and returns the steps.
    pub fn get_steps(self) -> Vec<Vec<Step>> {
        self.steps
    }
}
