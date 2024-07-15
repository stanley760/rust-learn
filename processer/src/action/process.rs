use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub struct Process<'a> {
    protocol: &'a str,

    innert_host: &'a str,

    outer_host: &'a str,

    status: &'a str,

    pid: &'a str,
}

impl Process<'_> {
    pub fn run() -> Vec<Self> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "netstat -ano"]).output().expect("failed to run cmd")
        } else if cfg!(target_os="macos") {
            Command::new("sh").args(&["-c", "netstat -anv"]).output().expect("failed to run in mac")
        } else {
            Command::new("sh").args(&["-c", "netstat -ano"]).output().expect("failed to run sh")
        };
        let output = String::from_utf8_lossy(&output.stdout);
        println!("{}", output);
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Process::run();
        assert_eq!(result.len(), 0);
    }
}