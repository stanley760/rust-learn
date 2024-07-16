use iced::{Alignment, Element, Length, Sandbox};
use iced::alignment::Horizontal::Center;
use iced::alignment::Vertical;
use iced::widget::{button, Button, Column, Row, Text, text, text_input, TextInput};

#[derive(Default)]
pub struct TableList {
    port_in_val: u32,
    pid_in_val: u32,
    btn_search: button::State,
    btn_reset: button::State,
    btn_kill: button::State,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Search(u32),
    Reset,
    Kill(u32),
}

impl Sandbox for TableList {
    type Message = Message;

    fn new() -> Self {
        Self {
            port_in_val: 0,
            pid_in_val: 0,
            btn_search: Default::default(),
            btn_reset: Default::default(),
            btn_kill: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("process killer")
    }

    fn update(&mut self, message: Self::Message) {
       
    }

    fn view(&self) -> Element<'_, Self::Message> {
       // todo optimize the code
        let port_text = text("port:").size(20).vertical_alignment(Vertical::Center);
        let port_input = text_input("please input the port numb"
                                    , &mut self.port_in_val.to_string()).padding(10);

        let btn_search = button("search")
            .padding(10)
            .on_press(Message::Search(self.port_in_val));

        let btn_reset = button("reset")
            .padding(10)
            .on_press(Message::Reset);

        let pid_text = text("pid:").size(20).vertical_alignment(Vertical::Center);

        let pid_input = text_input("please input the pid numb"
                                   , &mut self.pid_in_val.to_string()).padding(10);

        let btn_kill = button("kill")
            .padding(10)
            .on_press(Message::Kill(self.pid_in_val));

        Column::new().push(
            Row::new().push(port_text)
                .push(port_input)
                .push(btn_search)
                .push(btn_reset)
                .push(pid_text)
                .push(pid_input)
                .push(btn_kill))
            .spacing(20)
            .height(100)
            .padding(20)
            .max_width(800)
            .align_items(Alignment::Center)
            .into()
    }
}