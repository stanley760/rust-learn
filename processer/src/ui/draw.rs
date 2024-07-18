use iced::{Alignment, Element, Length, Sandbox};
use iced::alignment::Vertical;
use iced::widget::{button, Button, Column, Row, Space, Text, TextInput};

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
        let port_text = Text::new("port:").size(20).vertical_alignment(Vertical::Center);
        let port_input = TextInput::new("input port numb"
                                    , & self.port_in_val.clone())
            .on_input(Message::SearchInputChanged).padding(5);

        let btn_search = Button::new("search")
            .padding(5)
            .on_press(Message::Search);

        let btn_reset = Button::new("reset")
            .padding(5)
            .on_press(Message::Reset);

        let pid_text = Text::new("pid:").size(20).vertical_alignment(Vertical::Center);

        let pid_input = TextInput::new("input pid numb"
                                   , &self.pid_in_val)
            .on_input(Message::KillInputChanged)
            .padding(5);

        let btn_kill = Button::new("kill")
            .padding(5)
            .width(Length::Fixed(35f32))
            .on_press(Message::Kill);
        
        
        
        // todo table view
        Column::new().push(
            Row::new().push(port_text)
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
                .push(btn_reset))
            
            .spacing(10)
            .height(100)
            .padding(20)
            .max_width(800)
            .align_items(Alignment::Center)
            .into()
    }
}