use std::fs::File;

use iced::{
    executor,
    Application, Command, Rectangle, Renderer, Subscription, Theme,
};

use crate::alarm::Alarm;

const TICK_DURATION_SECS: u64 = 5;

pub struct App {
    alarms: Vec<Alarm>,
    config: File,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = (Vec<Alarm>, File);
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (alarms, config) = flags;

        (Self { alarms, config }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Process Alarm")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::None => (),
            Message::Tick => (),
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        todo!()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use iced::time as itime;

        let duration = itime::Duration::from_secs(TICK_DURATION_SECS);
        itime::every(duration).map(|_| Message::Tick)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    None,
    Tick,
}

