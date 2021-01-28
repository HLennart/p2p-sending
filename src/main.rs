mod contact_list;

use contact_list::{ContactList, ContactMessage};
use iced::{Application, Color, Column, Command, Container, Element, Settings, Text, button, executor};

pub fn main() -> iced::Result {
    env_logger::init();

    App::run(Settings::default())
}

pub struct App {
    steps: Steps,
    back_button: button::State,
    next_button: button::State,
    contact_list: ContactList,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                steps: Steps::new(),
                back_button: button::State::new(),
                next_button: button::State::new(),
                contact_list: ContactList::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("NICE - {}", self.steps.title())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::BackPressed => {
                // self.steps.go_back();
            }
            Message::NextPressed => {
                // self.steps.advance();
            }
            Message::ContactMessage(message) => {
                self.contact_list.update(message);
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let x: Element<_> = Column::new()
            .push(
        self.contact_list
                .view()
                .map(move |message| Message::ContactMessage(message)))
            .push(Text::new("hEYA!"))
        .into();

        x.explain(Color::BLACK)
        

        // Text::new("nice!").size(60).into()
    }
}

#[derive(Debug)]
pub enum Message {
    BackPressed,
    NextPressed,
    ContactMessage(ContactMessage),
}

pub enum StepMessage {
    DebugToggled(bool),
}

trait Page {
    fn update(&mut self, msg: StepMessage);
    fn view(&mut self) -> Element<StepMessage>;
    fn title(&self) -> &str;
}

struct Steps {
    steps: Vec<Box<dyn Page>>,
    current: usize,
}

impl Steps {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() //&& self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        "123"
        // self.steps[self.current].title()
    }
}

enum Step {}
