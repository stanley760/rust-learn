use crate::action::process::Process;
use dioxus::prelude::*;

pub fn app() -> Element {
    let header_txt: [&str; 5] = ["protocol", "inner_host", "outer_host", "status", "pid"];
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
    // 提供上下文
    use_context_provider(|| error_message);
    rsx!(
        title { "Process Killer" }
        body {
            link { rel: "stylesheet", href: "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"}
            link { rel: "stylesheet", href: "process_killer/assets/style.css"}
            // 显示错误消息
            if !error_message.read().is_empty() {
                div { class: "error-message", "{error_message}" }
            }

            div {
                class: "div-form",
                label { form: "port-label", class: "port", "port:"}
                input { form: "port-input", name: "port-input", value: "{port_value}"
                    , oninput: move |event| {
                    port_value.set(event.value().clone())
                }, class: "port"}
                button { form: "btn-search", name: "btn-search", onclick: move |_| {
                    let port_str = port_value.read();
                    let data = match Process::search(port_str.as_str()) {
                        Ok(processes) => processes,
                        Err(e) => {
                            // 使用dioxus message tip组件显示错误信息到页面
                            error_message.set(format!("搜索进程时出错: {}", e));
                            return;
                        }
                    };
                    error_message.set(String::new());
                    datas.set(data.iter().map(|x| {
                        vec![x.protocol.clone(), x.inner_host.clone(), x.outer_host.clone(), x.status.clone(), x.pid.clone()]
                    }).collect());
                }, "search"}
                button { form: "btn-reset", name: "btn-reset", onclick: move |_| {
                    port_value.set(String::new()); pid_value.set(String::new())
                }, "reset" }

                label { form: "pid-label", "pid:"}
                input { form: "pid-input", name: "pid-input",value: "{pid_value}",
                    oninput: move |event| {
                        pid_value.set(event.value().clone())
                    }, class: "pid"}
                button { form: "btn-kill", name: "btn-kill", onclick: move |_| {
                    let pid_str = pid_value.read();
                    if let Err(e) = Process::kill(pid_str.as_str()) {
                       error_message.set(format!("终止进程时出错: {}", e));
                    } else {
                        // 成功终止进程后清除PID输入
                        error_message.set(String::new());
                    }

                }, "kill"}
            }

            table {
                thead {
                    tr {{ header_txt.iter().map(|header_text| rsx!{th { "{header_text}" }}) }}
                }

                tbody {
                    {datas.iter().map(|data| rsx! {tr {
                        {data.iter().map(|x| rsx!{td { "{x}" }})}
                    }})}
                }
            }
        }
    )
}
