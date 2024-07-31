use dioxus::prelude::*;

//use crate::action::process::Process;

// const TABLE_HEAD_FONT_SIZE: u16 = 18;
// const COLUMN_WIDTH_PORTION: u16 = 10;
// const HEADER_TEXT: [&'static str; 5] = ["protocol", "inner_host", "outer_host", "status", "pid"];
// 
// #[allow(dead_code)]
// pub struct TableList {
//     port_in_val: String,
//     pid_in_val: String,
//     btn_search: button::State,
//     btn_reset: button::State,
//     btn_kill: button::State,
//     error_message: Option<String>,
// }
// 
// #[derive(Debug, Clone)]
// pub enum Message {
//     SearchInputChanged(String),
//     Reset,
//     KillInputChanged(String),
//     Search,
//     Kill,
// }
// 
// impl Application for TableList {
//     type Executor = iced::executor::Default;
//     type Message = Message;
//     type Theme = iced::Theme;
//     type Flags = ();
// 
//     fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
//         (TableList {
//             port_in_val: String::new(),
//             pid_in_val: String::new(),
//             btn_search: Default::default(),
//             btn_reset: Default::default(),
//             btn_kill: Default::default(),
//             error_message: None,
//         }, Command::none())
//     }
// 
//     fn title(&self) -> String {
//         String::from("process killer")
//     }
// 
//     fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
//         match message {
//             Message::SearchInputChanged(val) => {
//                 self.port_in_val = val;
//             }
//             Message::Reset => {
//                 self.port_in_val.clear();
//                 self.pid_in_val.clear();
//             }
//             Message::KillInputChanged(val) => {
//                 self.pid_in_val = val;
//             }
//             Message::Search => {
//                 // Handle search button press
//                 println!("Searching for port {}", self.port_in_val);
//             }
//             Message::Kill => {
//                 // Handle kill button press
// 
//                 match Process::kill(&self.pid_in_val) {
//                     Err(err) => {
//                         self.error_message = Some(format!("Failed to kill process: {}", err));
//                     }
//                     _ => {}
//                 }
//             }
//         };
//         Command::none()
//     }
// 
// 
//     fn view(&self) -> Element<'_, Self::Message> {
//         let port_text = text("port:").size(20).vertical_alignment(Vertical::Center);
//         let port_input = text_input("input port numb", &self.port_in_val.clone())
//             .on_input(Message::SearchInputChanged).padding(5);
// 
//         let btn_search = button("search")
//             .padding(5)
//             .on_press(Message::Search);
// 
//         let btn_reset = button("reset")
//             .padding(5)
//             .on_press(Message::Reset);
// 
//         let pid_text = text("pid:").size(20).vertical_alignment(Vertical::Center);
// 
//         let pid_input = text_input("input pid numb", &self.pid_in_val)
//             .on_input(Message::KillInputChanged)
//             .padding(5);
// 
//         let btn_kill = button("kill")
//             .padding(5)
//             .width(Length::Fixed(35f32))
//             .on_press(Message::Kill);
// 
//         // let message = Row::new().push(
//         //     match &self.error_message {
//         //         Some(error_message) => {
//         //             Text::new(error_message).size(15)
//         //         }
//         //         None => {
//         //             Text::new("")
//         //         }
//         //     });
// 
//         let row = Row::new().push(port_text)
//             .push(port_input)
//             .push(btn_search)
//             .push(Space::new(10, 0))
//             .push(pid_text)
//             .push(pid_input)
//             .push(btn_kill)
//             .push(btn_reset)
//             .spacing(10)
//             .height(80)
//             .padding(15)
//             .width(Length::Fill);
// 
// 
//         let table_header = Row::new()
//             .extend(HEADER_TEXT.iter().map(|header_text| {
//                 text(header_text)
//                     .size(TABLE_HEAD_FONT_SIZE)
//                     .width(Length::FillPortion(COLUMN_WIDTH_PORTION))
//                     .horizontal_alignment(Horizontal::Center).into()
//             })).width(Length::Fill).align_items(Alignment::Center);
// 
//         let mut table_column = Column::new()
//             .align_items(Alignment::Center);
// 
//         let datas: Vec<Vec<String>> = Process::run().iter().map(|x| {
//             vec![x.protocol.clone(), x.inner_host.clone(), x.outer_host.clone(), x.status.clone(), x.pid.clone()]
//         }).collect();
// 
//         let table_body: Vec<Row<_>> = datas.iter().map(|data| {
//             Row::new().extend(data.iter()
//                 .map(|x| text(x)
//                     .width(Length::FillPortion(COLUMN_WIDTH_PORTION))
//                     .horizontal_alignment(Horizontal::Center).into()))
//                 .width(Length::Fill).align_items(Alignment::Center).into()
//         }).collect::<Vec<_>>();
// 
//         for row in table_body {
//             table_column = table_column.push(row);
//         }
// 
//         Column::new()
//             .push(row)
//             .push(table_header)
//             .push(scrollable(table_column).height(Length::Shrink).width(Length::Fill))
//             .align_items(Alignment::Center)
//             .into()
//     }
// }

pub fn app() -> Element {
    rsx!("hello world")
}