use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::str;
use sysinfo::{Pid, System};

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
            Command::new("cmd")
                .args(["/C", "netstat -ano"])
                .output()
                .expect("failed to run cmd")
        } else if cfg!(target_os = "macos") {
            Command::new("sh")
                .args(["-c", "netstat -anv"])
                .output()
                .expect("failed to run in mac")
        } else {
            Command::new("sh")
                .args(["-c", "netstat -antp 2>/dev/null || netstat -an"])
                .output()
                .expect("failed to run sh")
        };

        let output = String::from_utf8_lossy(&output.stdout);

        output
            .lines()
            .filter_map(|x| {
                let parts: Vec<&str> = x.split_whitespace().collect();
                Self::handle_cross_operate_system(parts)
            })
            .skip(1)
            .collect()
    }

    pub fn search(port: &str) -> Result<Vec<Self>, Error> {
        if port.trim().is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "端口号不能为空"));
        }

        let port = port
            .parse::<u32>()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "端口号必须是数字"))?;

        let mut result = Self::run();
        result.retain(|x| {
            let inner_host = &x.inner_host;
            let port_str = if inner_host.starts_with('[') {
                inner_host.find("]:").map(|pos| &inner_host[pos + 2..])
            } else {
                inner_host.find(':').map(|pos| &inner_host[pos + 1..])
            };

            if let Some(port_str) = port_str {
                match port_str.parse::<u32>() {
                    Ok(parsed_port) => parsed_port == port,
                    Err(_) => false,
                }
            } else {
                false
            }
        });

        if result.is_empty() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("未找到使用端口 {} 的进程", port),
            ));
        }

        Ok(result)
    }

    pub fn kill(pid: &str) -> Result<(), Error> {
        if pid.trim().is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "PID 不能为空"));
        }

        let pid_num = pid
            .parse::<u32>()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "PID 必须是数字"))?;

        // 使用 sysinfo 进行跨平台进程终止
        let mut sys = System::new_all();
        sys.refresh_all();

        let pid_obj = Pid::from_u32(pid_num);

        if let Some(process) = sys.process(pid_obj) {
            if process.kill() {
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::PermissionDenied,
                    format!("无法终止进程 {} (可能需要管理员权限)", pid_num),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("未找到 PID 为 {} 的进程", pid_num),
            ))
        }
    }

    fn handle_cross_operate_system(parts: Vec<&str>) -> Option<Self> {
        let windows_flag = cfg!(target_os = "windows");
        let macos_flag = cfg!(target_os = "macos");

        if (parts.len() < 5 && windows_flag) || (macos_flag && parts.len() < 9) {
            return None;
        }

        let protocol = parts[0].to_string();
        let inner_host = if macos_flag {
            parts[3].to_string()
        } else {
            parts[1].to_string()
        };
        let outer_host = if macos_flag {
            parts[4].to_string()
        } else {
            parts[2].to_string()
        };
        let status = if macos_flag {
            parts[5].to_string()
        } else {
            parts[3].to_string()
        };
        let pid = if macos_flag {
            parts[8].to_string()
        } else if windows_flag {
            parts[4].to_string()
        } else {
            // Linux: 尝试从最后一列获取 PID
            parts.last().unwrap_or(&"").to_string()
        };

        Some(Self {
            protocol,
            inner_host,
            outer_host,
            status,
            pid,
        })
    }
}
