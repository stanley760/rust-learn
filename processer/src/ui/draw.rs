use iced::{Alignment, Element, Length, Sandbox};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, Row, Space, text, text_input};

use crate::action::process::Process;

const TABLE_HEAD_FONT_SIZE: u16 = 18;
const COLUMN_WIDTH_PORTION: u16 = 10;
const HEADER_TEXT: [&'static str; 5] = ["protocol", "inner_host", "outer_host", "status", "pid"];

#[derive(Default)]
pub struct TableList {
    port_in_val: String,
    pid_in_val: String,
    btn_search: button::State,
    btn_reset: button::State,
    btn_kill: button::State,
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
        let port_text = text("port:").size(20).vertical_alignment(Vertical::Center);
        let port_input = text_input("input port numb"
                                    , &self.port_in_val.clone())
            .on_input(Message::SearchInputChanged).padding(5);

        let btn_search = button("search")
            .padding(5)
            .on_press(Message::Search);

        let btn_reset = button("reset")
            .padding(5)
            .on_press(Message::Reset);

        let pid_text = text("pid:").size(20).vertical_alignment(Vertical::Center);

        let pid_input = text_input("input pid numb"
                                   , &self.pid_in_val)
            .on_input(Message::KillInputChanged)
            .padding(5);

        let btn_kill = button("kill")
            .padding(5)
            .width(Length::Fixed(35f32))
            .on_press(Message::Kill);

        let row = Row::new().push(port_text)
            .push(port_input)
            .push(btn_search)
            .push(Space::new(10, 0))
            .push(pid_text)
            .push(pid_input)
            .push(btn_kill)
            .push(btn_reset)
            .spacing(10)
            .height(80)
            .padding(15)
            .width(Length::Fill);


        let header = Row::new()
            .extend(HEADER_TEXT.iter().map(|header_text| {
            text(header_text)
                .size(TABLE_HEAD_FONT_SIZE)
                .width(Length::FillPortion(COLUMN_WIDTH_PORTION))
                .horizontal_alignment(Horizontal::Center).into()
        })).width(Length::Fill).align_items(Alignment::Center);

        // todo table view
        let datas: Vec<Vec<String>> = Process::run().iter().map(|x| {
            vec![x.protocol.clone(), x.inner_host.clone(), x.outer_host.clone(), x.status.clone(), x.pid.clone()]
        }).collect();


        // type AppRenderer = WgpuRenderer<Theme>;
        // let table_body = datas.iter().map(|data| {
        //     Row::new::<AppRenderer>()
        //         .extend(data.iter().map(|data| {
        //         text(data)
        //             .width(Length::FillPortion(COLUMN_WIDTH_PORTION))
        //             .into()
        //     })).width(Length::Fill)
        // }).collect::<Vec<_>>();

        column![row, header].align_items(Alignment::Center).into()
    }
}