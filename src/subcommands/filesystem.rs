use crate::command::cmd;
use crate::step::{ShouldRunResult, StepItem};
use async_trait::async_trait;
use tokio::process::Command;

pub struct CreateFile {
    file_path: String,
}

impl CreateFile {
    pub fn new(file_path: &str) -> CreateFile {
        CreateFile {
            file_path: file_path.to_string(),
        }
    }
}

#[async_trait]
impl StepItem for CreateFile {
    fn title(&self) -> String {
        format!("Creating File {}", self.file_path)
    }

    fn description(&self) -> String {
        "Creates a new file at the destination path.".to_string()
    }

    async fn should_run(&self) -> ShouldRunResult {
        let path_exists = std::path::Path::new(&self.file_path).exists();
        if path_exists {
            ShouldRunResult::Skip
        } else {
            ShouldRunResult::Ok
        }
    }

    ///
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let path = &self.file_path;
        cmd(
            || Command::new("touch").arg(path).output(),
            &format!("touch {}", path),
        )
        .await
    }
}

pub struct CopyFile {
    src_file_path: String,
    dst_file_path: String,
}

impl CopyFile {
    pub fn new(src_file_path: &str, dst_file_path: &str) -> CopyFile {
        CopyFile {
            src_file_path: src_file_path.to_string(),
            dst_file_path: dst_file_path.to_string(),
        }
    }
}

#[async_trait]
impl StepItem for CopyFile {
    fn title(&self) -> String {
        format!(
            "Copying File {} to {}",
            self.src_file_path, self.dst_file_path,
        )
    }

    fn description(&self) -> String {
        format!(
            "Copies the source file {} and creates the copy at {}.",
            self.src_file_path, self.dst_file_path,
        )
    }

    async fn should_run(&self) -> ShouldRunResult {
        let path_exists = std::path::Path::new(&self.dst_file_path).exists();
        if path_exists {
            ShouldRunResult::Skip
        } else {
            ShouldRunResult::Ok
        }
    }

    ///
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let src_path = &self.src_file_path;
        let dst_path = &self.dst_file_path;
        cmd(
            || Command::new("cp").arg(src_path).arg(dst_path).output(),
            &format!("cp {} {}", src_path, dst_path),
        )
        .await
    }
}
