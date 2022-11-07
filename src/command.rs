//! OeCli Subcommand executor.

use crate::log::LogLevel;
#[cfg(feature = "println")]
use crate::step::msg_handler::ConsoleLogEventHandler;
use crate::step::msg_handler::ProgressBarsEventHandler;
use crate::step::{ExecutorProperties, StepExecutor};
use async_trait::async_trait;
use std::future::Future;

/// Describes a way to define a series of steps to be executed and processes each one.
/// The implementation defines which steps are executed in what order and which can be run in
/// parallel.
#[async_trait]
pub trait CLIStepExecutor {
    /// You define the steps you want to be executed and what order. The ExecutorProperties is
    /// a provided struct that is expected to be returned by the implementation.
    ///
    /// Example:
    ///
    /// ```
    /// impl CLIStepExecutor for MyCLI {
    ///   async fn set_properties(&self, props: ExecutorProperties) -> ExecutorProperties {
    ///     let step1 = Step::Step(Box::new(MyStep1));
    ///     let step2 = Step::Step(Box::new(MyStep2));
    ///     let step3 = Step::Step(Box::new(MyStep3));
    ///     let step4 = Step::Step(Box::new(MyStep4));
    ///     props.run_parallel(step1, step2)
    ///       .then_run(step3)
    ///       .then_run(step4)
    ///   }
    /// }
    /// ```
    async fn set_properties(&self, props: ExecutorProperties) -> ExecutorProperties;

    /// Sets up the method to notify the caller based on cargo features. Calls the impl
    /// `set_properties` and uses that to define the behavior on how to execute all the steps.
    ///
    ///
    /// Two current supported methods for monitoring are, the default, progress bars and standard
    /// `println!` macro.
    async fn execute(&self) -> Result<(), String> {
        let msg_handler = ProgressBarsEventHandler::new(LogLevel::Info);
        #[cfg(feature = "println")]
        let msg_handler = ConsoleLogEventHandler::new(LogLevel::Info);
        let command_props = self.set_properties(ExecutorProperties::new()).await;
        StepExecutor::new(msg_handler)
            .build_steps(command_props)
            .run()
            .await?;
        Ok(())
    }
}

/// Simple wrapper around a `tokio::process::Command` that returns a
/// `Result<String, String>`.
///
/// Result will likely get refactored to be more useful.
pub async fn cmd<F, Fut>(fun: F, cmd_name: &str) -> Result<String, String>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<std::process::Output, std::io::Error>>,
{
    let result = fun().await;
    match result {
        Ok(output) => {
            if output.status.success() {
                return Ok(format!("Completed running `{}`.", cmd_name));
            } else {
                return Err(format!(
                    "Error running `{}`. stderr: {}",
                    cmd_name,
                    String::from_utf8(output.stderr).unwrap()
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "Failed to run `{}`. Error: {}",
                cmd_name,
                e.to_string()
            ));
        }
    }
}
