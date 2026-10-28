//! `WEB_SEARCH_MAX_RESULTS` 的环境变量解析测试。
//!
//! 单独成文件：环境变量是进程级全局状态，放进库内测试会与其它用例相互干扰。
//! 本文件内的用例共享同一变量，因此用一个互斥锁串行化。

use std::sync::{Mutex, MutexGuard};

use defined_agent::tools::web_search::websearch::{DEFAULT_MAX_RESULTS, max_results};

/// 串行化本文件内的环境变量用例。
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// 修改环境变量并读取解析结果，最后恢复原值。
fn with_env(value: Option<&str>, assert_fn: impl Fn(u8)) {
    let _guard: MutexGuard<()> = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let previous = std::env::var("WEB_SEARCH_MAX_RESULTS").ok();
    // SAFETY: 已通过 ENV_LOCK 串行化，本文件内不会与其他用例并发修改该变量。
    unsafe {
        match value {
            Some(raw) => std::env::set_var("WEB_SEARCH_MAX_RESULTS", raw),
            None => std::env::remove_var("WEB_SEARCH_MAX_RESULTS"),
        }
    }

    assert_fn(max_results());

    // SAFETY: 同上。
    unsafe {
        match previous {
            Some(raw) => std::env::set_var("WEB_SEARCH_MAX_RESULTS", raw),
            None => std::env::remove_var("WEB_SEARCH_MAX_RESULTS"),
        }
    }
}

#[test]
fn missing_env_uses_default() {
    with_env(None, |actual| assert_eq!(actual, DEFAULT_MAX_RESULTS));
}

#[test]
fn valid_env_overrides_default() {
    with_env(Some("7"), |actual| assert_eq!(actual, 7));
}

#[test]
fn whitespace_is_trimmed() {
    with_env(Some("  4  "), |actual| assert_eq!(actual, 4));
}

#[test]
fn invalid_values_fall_back_to_default() {
    for raw in ["0", "abc", "-1", "", "99999999999"] {
        with_env(Some(raw), |actual| {
            assert_eq!(
                actual, DEFAULT_MAX_RESULTS,
                "invalid value {raw:?} should fall back to default"
            )
        });
    }
}
