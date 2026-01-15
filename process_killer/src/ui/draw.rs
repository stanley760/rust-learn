use crate::action::process::Process;
use dioxus::prelude::*;

pub fn app() -> Element {
    let header_txt: [&str; 5] = ["协议", "本地地址", "远程地址", "状态", "PID"];
    let init_datas: Vec<Vec<String>> = Process::run()
        .iter()
        .map(|x| {
            vec![
                x.protocol.clone(),
                x.inner_host.clone(),
                x.outer_host.clone(),
                x.status.clone(),
                x.pid.clone(),
            ]
        })
        .collect();
    let mut datas = use_signal(|| init_datas);
    let mut port_value = use_signal(String::new);
    let mut pid_value = use_signal(String::new);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    let handle_search = move |_| {
        let port_str = port_value.read().clone();
        match Process::search(&port_str) {
            Ok(processes) => {
                datas.set(
                    processes
                        .iter()
                        .map(|x| {
                            vec![
                                x.protocol.clone(),
                                x.inner_host.clone(),
                                x.outer_host.clone(),
                                x.status.clone(),
                                x.pid.clone(),
                            ]
                        })
                        .collect(),
                );
                error_message.set(String::new());
                success_message.set(format!(
                    "找到 {} 个使用端口 {} 的进程",
                    processes.len(),
                    port_str
                ));
            }
            Err(e) => {
                error_message.set(format!("搜索失败: {}", e));
                success_message.set(String::new());
            }
        }
    };

    let handle_refresh = move |_| {
        let all_processes = Process::run();
        datas.set(
            all_processes
                .iter()
                .map(|x| {
                    vec![
                        x.protocol.clone(),
                        x.inner_host.clone(),
                        x.outer_host.clone(),
                        x.status.clone(),
                        x.pid.clone(),
                    ]
                })
                .collect(),
        );
        port_value.set(String::new());
        pid_value.set(String::new());
        error_message.set(String::new());
        success_message.set("已刷新进程列表".to_string());
    };

    let handle_reset = move |_| {
        port_value.set(String::new());
        pid_value.set(String::new());
        error_message.set(String::new());
        success_message.set(String::new());
    };

    let handle_kill = move |_| {
        let pid_str = pid_value.read().clone();
        match Process::kill(&pid_str) {
            Ok(_) => {
                success_message.set(format!("成功终止进程 {}", pid_str));
                error_message.set(String::new());
                pid_value.set(String::new());

                // 刷新进程列表
                let all_processes = Process::run();
                datas.set(
                    all_processes
                        .iter()
                        .map(|x| {
                            vec![
                                x.protocol.clone(),
                                x.inner_host.clone(),
                                x.outer_host.clone(),
                                x.status.clone(),
                                x.pid.clone(),
                            ]
                        })
                        .collect(),
                );
            }
            Err(e) => {
                error_message.set(format!("终止进程失败: {}", e));
                success_message.set(String::new());
            }
        }
    };

    rsx!(
        title { "Process Killer" }
        body {
            link { rel: "stylesheet", href: "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"}
            link { rel: "stylesheet", href: "process_killer/assets/style.css"}

            // 显示错误消息
            if !error_message.read().is_empty() {
                div { class: "error-message", "{error_message}" }
            }

            // 显示成功消息
            if !success_message.read().is_empty() {
                div { class: "success-message", "{success_message}" }
            }

            div {
                class: "div-form",

                label { r#for: "port-input", class: "port", "端口:"}
                input {
                    id: "port-input",
                    name: "port-input",
                    value: "{port_value}",
                    placeholder: "输入端口号",
                    oninput: move |event| {
                        port_value.set(event.value().clone());
                        error_message.set(String::new());
                        success_message.set(String::new());
                    },
                    class: "port"
                }

                button {
                    class: "btn-search",
                    onclick: handle_search,
                    "搜索"
                }

                button {
                    class: "btn-refresh",
                    onclick: handle_refresh,
                    "刷新"
                }

                button {
                    class: "btn-reset",
                    onclick: handle_reset,
                    "重置"
                }

                label { r#for: "pid-input", "PID:"}
                input {
                    id: "pid-input",
                    name: "pid-input",
                    value: "{pid_value}",
                    placeholder: "输入进程 PID",
                    oninput: move |event| {
                        pid_value.set(event.value().clone());
                        error_message.set(String::new());
                        success_message.set(String::new());
                    },
                    class: "pid"
                }

                button {
                    class: "btn-kill",
                    onclick: handle_kill,
                    "终止进程"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {{ header_txt.iter().map(|header_text| rsx!{th { "{header_text}" }}) }}
                    }

                    tbody {
                        {datas.read().iter().enumerate().map(|(idx, data)| {
                            let pid_data = data.last().cloned().unwrap_or_default();
                            rsx! {
                                tr {
                                    key: "{idx}",
                                    onclick: move |_| {
                                        pid_value.set(pid_data.clone());
                                    },
                                    {data.iter().map(|x| rsx!{td { "{x}" }})}
                                }
                            }
                        })}
                    }
                }
            }
        }
    )
}
