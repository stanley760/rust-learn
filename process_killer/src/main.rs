use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

use process_killer::ui::draw::app;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new()
                .with_title("process killer")
                .with_resizable(true)
            )
        ).launch(app)
}

