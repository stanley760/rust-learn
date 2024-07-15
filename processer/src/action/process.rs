use std::process::Command;
use std::str;

#[derive(Debug, Clone)]
pub struct Process {
    protocol: String,

    innert_host: String,

    outer_host: String,

    status: String,

    pid: String,
}

impl Process {
    pub fn run() -> Vec<Self> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "netstat -ano"]).output().expect("failed to run cmd")
        } else if cfg!(target_os="macos") {
            Command::new("sh").args(&["-c", "netstat -anv"]).output().expect("failed to run in mac")
        } else {
            Command::new("sh").args(&["-c", "netstat -ano"]).output().expect("failed to run sh")
        };
        let output = str::from_utf8(&output.stdout).expect("failed to convert output to utf-8");
        // todo return output.lines().skip(1)
        vec![]
    }
}

