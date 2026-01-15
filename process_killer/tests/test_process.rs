#[cfg(test)]
mod tests {
    use process_killer::action::process::Process;

    #[test]
    fn test_run_processes() {
        let result = Process::run();
        println!("找到 {} 个网络进程", result.len());
        assert!(!result.is_empty(), "应该至少有一些网络进程");

        // 打印前 5 个进程作为示例
        for (i, process) in result.iter().take(5).enumerate() {
            println!("进程 {}: {:?}", i + 1, process);
        }
    }

    #[test]
    fn test_search_invalid_port() {
        // 测试无效的端口号
        let result = Process::search("");
        assert!(result.is_err(), "空端口号应该返回错误");

        let result = Process::search("abc");
        assert!(result.is_err(), "非数字端口号应该返回错误");

        let result = Process::search("99999");
        // 这个可能成功也可能失败，取决于是否有进程使用该端口
        println!("搜索端口 99999 的结果: {:?}", result);
    }

    #[test]
    fn test_kill_invalid_pid() {
        // 测试无效的 PID
        let result = Process::kill("");
        assert!(result.is_err(), "空 PID 应该返回错误");

        let result = Process::kill("abc");
        assert!(result.is_err(), "非数字 PID 应该返回错误");

        // 测试不存在的 PID（使用一个很大的数字）
        let result = Process::kill("999999");
        assert!(result.is_err(), "不存在的 PID 应该返回错误");
    }

    #[test]
    fn test_process_structure() {
        let processes = Process::run();

        if let Some(first) = processes.first() {
            println!("第一个进程:");
            println!("  协议: {}", first.protocol);
            println!("  本地地址: {}", first.inner_host);
            println!("  远程地址: {}", first.outer_host);
            println!("  状态: {}", first.status);
            println!("  PID: {}", first.pid);

            // 验证字段不为空
            assert!(!first.protocol.is_empty(), "协议不应为空");
            assert!(!first.inner_host.is_empty(), "本地地址不应为空");
            assert!(!first.pid.is_empty(), "PID 不应为空");
        }
    }
}
