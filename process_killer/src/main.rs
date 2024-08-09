use std::env;

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::desktop::tao::window::Icon;
use dioxus::prelude::*;

use process_killer::ui::draw::app;

fn main() {
    let buf = env::current_dir().unwrap();
    let exe_dir = buf.join("process_killer");
    let icon_path = exe_dir.join("src").join("assets").join("icon.png");
    let icon = image::open(icon_path).expect("Failed to load icon file");

    let window = WindowBuilder::new()
        .with_title("process killer")
        .with_window_icon(Some(Icon::from_rgba(icon.to_rgba8().into_raw(), icon.width(), icon.height()).unwrap()))
        .with_resizable(true);


    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app)
}

