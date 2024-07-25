use iced::{Alignment, Element, Length, Sandbox};
use iced::alignment::Vertical;
use iced::widget::{button, Button, Column, Row, Space, Text, TextInput};

use crate::action::process::Process;

#[derive(Default)]
pub struct TableList {
    port_in_val: String,
    pid_in_val: String,
    btn_search: button::State,
    btn_reset: button::State,
    btn_kill: button::State,
    //  table: Table,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchInputChanged(String),
    Reset,
    KillInputChanged(String),
    Search,
    Kill,
}

impl Sandbox for TableList {
    type Message = Message;

    fn new() -> Self {
        Self {
            port_in_val: String::new(),
            pid_in_val: String::new(),
            btn_search: Default::default(),
            btn_reset: Default::default(),
            btn_kill: Default::default(),
            //druid_container: ,
        }
    }

    fn title(&self) -> String {
        String::from("process killer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SearchInputChanged(val) => {
                self.port_in_val = val;
            }
            Message::Reset => {
                self.port_in_val.clear();
                self.pid_in_val.clear();
            }
            Message::KillInputChanged(val) => {
                self.pid_in_val = val;
            }
            Message::Search => {
                // Handle search button press
                println!("Searching for port {}", self.port_in_val);
            }
            Message::Kill => {
                // Handle kill button press
                println!("Killing process with pid {}", self.pid_in_val);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let port_text = Text::new("port:").size(20).vertical_alignment(Vertical::Center);
        let port_input = TextInput::new(""
                                        , &self.port_in_val.clone())
            .on_input(Message::SearchInputChanged).padding(5);

        let btn_search = Button::new("search")
            .padding(5)
            .on_press(Message::Search);

        let btn_reset = Button::new("reset")
            .padding(5)
            .on_press(Message::Reset);

        let pid_text = Text::new("pid:").size(20).vertical_alignment(Vertical::Center);

        let pid_input = TextInput::new("" , &self.pid_in_val)
            .on_input(Message::KillInputChanged)
            .padding(5);

        let btn_kill = Button::new("kill")
            .padding(5)
            .width(Length::Fixed(35f32))
            .on_press(Message::Kill);

        let search = Row::new().push(port_text)
            .push(Space::new(5, 0))
            .push(port_input)
            .push(Space::new(10, 0))
            .push(btn_search)
            .push(Space::new(10, 0))

            .push(Space::new(10, 0))
            .push(pid_text)
            .push(Space::new(5, 0))
            .push(pid_input)
            .push(Space::new(10, 0))
            .push(btn_kill)
            .push(Space::new(10, 0))
            .push(btn_reset);

        let header: Row<_, _, _> = Row::new().push(Text::new("协议"))
            .push(Text::new("内部域名"))
            .push(Text::new("外部域名"))
            .push(Text::new("状态"))
            .push(Text::new("进程标识"));
        
        let datas: Vec<Vec<String>> = Process::run()
            .into_iter()
            .map(|data| vec![
                data.protocol.clone(),
                data.innert_host.clone(),
                data.outer_host.clone(),
                data.status.clone(),
                data.pid.clone(),
            ]).collect();

        let mut table = Column::new()
            .push(header)
            .padding(10)
            .spacing(10);

        for row in &datas {
            let mut data_row = Row::new();
            for item in row {
                data_row = data_row.push(Text::new(item.clone()));
            }
            table = table.push(data_row);
        }

        let row = Row::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(table);

        let ui = Column::new().push(search)
            .push(row)
            .spacing(10)
            .height(100)
            .padding(20)
            .max_width(800)
            .align_items(Alignment::Center);
        
        ui.into()
    }
}