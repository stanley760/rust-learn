use crate::error::ProcessError;
use sysinfo::{Pid, ProcessesToUpdate, System};

/// SysInfo Wrapper
/// 封装 sysinfo 库操作，提供跨平台进程管理功能
pub struct SysInfoWrapper {
    system: System,
}

impl SysInfoWrapper {
    /// 创建新的 SysInfoWrapper 实例
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// 刷新系统进程信息
    pub fn refresh(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
    }

    /// 终止指定 PID 的进程
    ///
    /// # Arguments
    /// * `pid` - 要终止的进程 ID
    ///
    /// # Returns
    /// * `Ok(())` - 进程成功终止
    /// * `Err(ProcessError::ProcessNotFound)` - 进程不存在
    /// * `Err(ProcessError::KillFailed)` - 进程终止失败（可能需要管理员权限）
    pub fn kill_process(&mut self, pid: u32) -> Result<(), ProcessError> {
        // 刷新进程信息以确保获取最新状态
        self.refresh();

        // 检查进程是否存在
        if !self.process_exists(pid) {
            return Err(ProcessError::ProcessNotFound(pid));
        }

        // 尝试终止进程
        let pid_obj = Pid::from_u32(pid);
        if let Some(process) = self.system.process(pid_obj) {
            if process.kill() {
                Ok(())
            } else {
                Err(ProcessError::KillFailed(pid))
            }
        } else {
            Err(ProcessError::ProcessNotFound(pid))
        }
    }

    /// 检查指定 PID 的进程是否存在
    ///
    /// # Arguments
    /// * `pid` - 要检查的进程 ID
    ///
    /// # Returns
    /// * `true` - 进程存在
    /// * `false` - 进程不存在
    pub fn process_exists(&self, pid: u32) -> bool {
        let pid_obj = Pid::from_u32(pid);
        self.system.process(pid_obj).is_some()
    }
}

impl Default for SysInfoWrapper {
    fn default() -> Self {
        Self::new()
    }
}
