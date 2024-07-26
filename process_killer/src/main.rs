use iced::{Application, Settings};
use process_killer::action::process::Process;
use process_killer::ui::draw::TableList;

fn main() -> iced::Result {
    Process::kill("123").unwrap_or_else(|err| println!("{}", err));
    TableList::run(Settings::default())
    
}

