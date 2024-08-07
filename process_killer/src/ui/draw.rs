use dioxus::prelude::*;
use crate::action::process::Process;

pub fn app() -> Element {
    let header_txt : [&str; 5] = ["protocol", "inner_host", "outer_host", "status", "pid",];
    
    let datas: Vec<Vec<String>> = Process::run().iter().map(|x| {
        vec![x.protocol.clone(), x.inner_host.clone(), x.outer_host.clone(), x.status.clone(), x.pid.clone()]
    }).collect();
    
    rsx!(
        title { "Process Killer" }
        body {
            link { rel: "stylesheet", href: "../../src/assets/style.css"}
            div {
                class: "div-form",
                label { form: "port-label", "port:"}
                input { form: "port-input", name: "port-input", class: "port-input"}
                button { form: "btn-search", name: "btn-search", "search"}
                button { form: "btn-reset", name: "btn-reset", "reset" }
                
                label { form: "pid-label", "pid:"}
                input { form: "pid-input", name: "pid-input", class: "pid-input"}
                button { form: "btn-kill", name: "btn-kill", "kill"}
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