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
        let (output, numb) = if cfg!(target_os = "windows") {
            (Command::new("cmd").args(&["/C", "netstat -ano"]).output().expect("failed to run cmd"), 1)
        } else if cfg!(target_os="macos") {
            (Command::new("sh").args(&["-c", "netstat -anv"]).output().expect("failed to run in mac"), 2)
        } else {
            (Command::new("sh").args(&["-c", "netstat -ano"]).output().expect("failed to run sh"), 1)
        };

        let output = String::from_utf8_lossy(&output.stdout);

        output.lines().filter_map(|x| {
            let parts: Vec<&str> = x.split_whitespace().collect();
            Self::hanle_cross_operate_system(parts)
        }).skip(numb).collect()
    }

    fn hanle_cross_operate_system(parts: Vec<&str>) -> Option<Process> {
        let windows_flag = cfg!(target_os = "windows");
        let macos_flag = cfg!(target_os = "macos");
        if (parts.len() < 5 && windows_flag) || (macos_flag && parts.len() < 8) {
            return None;
        }
        let protocol = parts[0].to_string();
        let innert_host = if macos_flag { parts[3].to_string() } else { parts[1].to_string() };
        let outer_host = if macos_flag { parts[4].to_string() } else { parts[2].to_string() };
        let status = if macos_flag { parts[5].to_string() } else { parts[3].to_string() };
        let pid = if macos_flag { parts[8].to_string() } else { parts[4].to_string() };

        Some(Process {
            protocol,
            innert_host,
            outer_host,
            status,
            pid,
        })
    }
}

