use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::str;

#[derive(Debug, Clone)]
pub struct Process {
    pub protocol: String,

    pub inner_host: String,

    pub outer_host: String,

    pub status: String,

    pub pid: String,
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

        let output = String::from_utf8_lossy(&output.stdout);

        output.lines().filter_map(|x| {
            let parts: Vec<&str> = x.split_whitespace().collect();
            Self::hanle_cross_operate_system(parts)
        }).skip(1).collect()
    }

    pub fn kill(pid: &str) -> Result<(), Error> {
        
        let pid = pid.parse::<u32>().map_err(|_| Error::new(ErrorKind::InvalidInput, "it's not a number"))?;
        
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["taskkill", "-PID", &pid.to_string(), "-F"])
                .output()
            // "failed to run on windows."
        } else {
            Command::new("sh")
                .args(&["-kill", "-9", pid.to_string().as_str()])
                .output()
            // "failed to run on linux/mac."
        };
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(ErrorKind::Other, format!("failed to execute the kill process: {}", e))),
        }
    }

    fn hanle_cross_operate_system(parts: Vec<&str>) -> Option<Process> {
        let windows_flag = cfg!(target_os = "windows");
        let macos_flag = cfg!(target_os = "macos");
        if (parts.len() < 5 && windows_flag) || (macos_flag && parts.len() < 8) {
            return None;
        }
        let protocol = parts[0].to_string();
        let inner_host = if macos_flag { parts[3].to_string() } else { parts[1].to_string() };
        let outer_host = if macos_flag { parts[4].to_string() } else { parts[2].to_string() };
        let status = if macos_flag { parts[5].to_string() } else { parts[3].to_string() };
        let pid = if macos_flag { parts[8].to_string() } else { parts[4].to_string() };

        Some(Process {
            protocol,
            inner_host,
            outer_host,
            status,
            pid,
        })
    }
}

