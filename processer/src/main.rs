use iced::{Sandbox, Settings};
use processer::action::process::Process;
use processer::ui::draw::TableList;

fn main() -> iced::Result {
    let vec = Process::run();
    println!("{:#?}", vec);
    TableList::run(Settings::default())
    
}

