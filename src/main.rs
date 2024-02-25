use std::{thread, time};
use enigo::{Enigo, Key, KeyboardControllable};
use iced::{widget, Sandbox, Element, Settings};

fn generate_text(content: String, delay: u8) {
    let time_delay = time::Duration::from_secs(delay as u64);
    thread::sleep(time_delay);

    let mut enigo = Enigo::new();

    for (idx, char) in content.to_lowercase().chars().enumerate() {
        let ch = content.chars().nth(idx).unwrap();

        match ch {
            '\n' => {
                enigo.key_click(Key::Return)
            },
            '~'| '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '('| ')' | '_' | '+' | '{' | '}' | '|' | ':' | '"' | '<' | '>' | '?'  => {
                println!("{}", ch);
                enigo.key_click(Key::Shift);
                enigo.key_click(Key::Layout(char));
            },
            _ => {
                if ch.is_uppercase() {
                    enigo.key_click(Key::CapsLock);
                }

                enigo.key_click(Key::Layout(char));

                if ch.is_uppercase(){
                    enigo.key_click(Key::CapsLock);
                }
            }
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    content: widget::text_editor::Content,
    delay: Option<u8>,
}

#[derive(Debug, Clone)]
enum Message {
    SelectDelay(u8),
    RunProgram,
    Edit(widget::text_editor::Action),
}


impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: widget::text_editor::Content::new(),
            delay: Some(3),
        }
    }

    fn title(&self) -> String {
        String::from("Tarzan")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectDelay(delay) => {
                self.delay = Some(delay);
            },
            Message::RunProgram => {
                generate_text(self.content.text(), self.delay.unwrap_or_else(|| 3));
            },
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let delay_list = widget::pick_list([2, 3, 5,10], self.delay, Message::SelectDelay).placeholder("Select delay");

        let button = widget::button("Run").on_press(Message::RunProgram);

        let text_editor = widget::text_editor(&self.content).height(500).on_action(Message::Edit);


        widget::container(widget::column![
            widget::row![
                delay_list,
                widget::horizontal_space().width(10),
                button
            ],
            widget::vertical_space().height(10),
            text_editor
        ]).padding(10).into()
    }
}