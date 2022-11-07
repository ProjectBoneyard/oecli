//! Optional cargo features enables this Subcommand. This subcommand is used for testing and
//! development purposes only.

use crate::command::CLIStepExecutor;
use crate::step::event::{EventData, NewSequenceEvent};
use crate::step::msg_handler::EventHandler;
use crate::step::{ExecutorProperties, ShouldRunResult, Step, StepDetails, StepItem, StepSequence};
use async_trait::async_trait;
use clap::Args;

/// Since the Event Handler is behind a lock, and we are intentionally timing out the threads, we can
/// deterministically test the timing of the messages to test the executor. This should move out
/// and under Event Handler behind the "tests" feature.
pub struct TestEvent {
    cb: fn(msg: String),
}

impl EventHandler for TestEvent {
    fn sequence_start(&mut self, details: NewSequenceEvent) {}
    fn sequence_end(&mut self, details: &EventData) {}
    fn step_start(&mut self, step: StepDetails, data: &EventData) {}
    fn step_end(&mut self, step: StepDetails, data: &EventData) {}
    fn step_skipped(&mut self, step: StepDetails, data: &EventData) {}
    fn step_error(&mut self, step: StepDetails, data: &EventData) {}
}

impl TestEvent {
    #[allow(dead_code)]
    pub fn new(cb: fn(msg: String)) -> TestEvent {
        TestEvent { cb }
    }
}

#[derive(Args, Clone, Debug)]
pub struct Test {
    /// Name of cloud, will become the git repository name.
    #[clap(long)]
    name: String,
    /// If the newly created repository should be public. Defaults to private.
    #[clap(long, short)]
    desc: String,
}

struct TestStep {
    num: usize,
    delay: usize,
}

impl TestStep {
    pub fn new(num: usize, delay: usize) -> TestStep {
        TestStep { num, delay }
    }
}

#[async_trait]
impl StepItem for TestStep {
    fn title(&self) -> String {
        format!("Test Step ({})", self.num)
    }

    fn description(&self) -> String {
        format!("Will sleep for {}ms", self.delay)
    }

    async fn should_run(&self) -> ShouldRunResult {
        ShouldRunResult::Ok
    }

    async fn execute(self: Box<Self>) -> Result<String, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.delay as u64)).await;
        Ok("".to_string())
    }
}

struct TestStep2 {
    num: usize,
    delay: usize,
}

impl TestStep2 {
    pub fn new(num: usize, delay: usize) -> TestStep2 {
        TestStep2 { num, delay }
    }
}

#[async_trait]
impl StepItem for TestStep2 {
    fn title(&self) -> String {
        format!("Step Step 2 ({})", self.num)
    }

    fn description(&self) -> String {
        format!("Will sleep for {}ms", self.delay)
    }

    async fn should_run(&self) -> ShouldRunResult {
        ShouldRunResult::Ok
    }

    async fn execute(self: Box<Self>) -> Result<String, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.delay as u64)).await;
        Ok("".to_string())
    }
}

#[derive(Args, Clone, Debug)]
pub struct TestCommand {}

#[async_trait]
impl CLIStepExecutor for TestCommand {
    async fn set_properties(&self, cmd_props: ExecutorProperties) -> ExecutorProperties {
        let one_sec = 1000;
        let t1 = TestStep::new(1, one_sec);
        let t2 = TestStep::new(2, one_sec);
        let t3 = TestStep::new(3, one_sec);
        let t4 = TestStep::new(4, one_sec);
        let t5 = TestStep::new(5, one_sec);
        let t6 = TestStep::new(6, one_sec);
        let t7 = TestStep::new(7, one_sec);
        let t8 = TestStep::new(8, one_sec);
        let t9 = TestStep::new(9, one_sec);
        let t10 = TestStep::new(10, one_sec);
        let t11 = TestStep2::new(11, one_sec);

        let seq_two = StepSequence::new("test sequence two", "")
            .then_run(Step::Step(Box::new(t5)))
            .then_run(Step::Step(Box::new(t6)))
            .then_run(Step::Step(Box::new(t7)))
            .then_run(Step::Step(Box::new(t8)));

        let seq_one = StepSequence::new("Test Sequence one", "")
            .then_run(Step::Step(Box::new(t3)))
            .then_run(Step::Step(Box::new(t4)))
            .then_run_parallel(vec![Step::Step(Box::new(t9)), Step::Sequence(seq_two)])
            .then_run(Step::Step(Box::new(t10)))
            .then_run(Step::Step(Box::new(t11)));

        cmd_props
            .then_run_parallel(vec![Step::Step(Box::new(t1)), Step::Step(Box::new(t2))])
            .then_run(Step::Sequence(seq_one))
    }
}
