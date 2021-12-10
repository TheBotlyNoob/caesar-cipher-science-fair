// Trying to decode and encode a caesar cipher with a GUI

// #![windows_subsystem = "windows"]
use iced::{
  executor, widget, Application, Clipboard, Column, Command, Element, Result, Settings, TextInput,
};
use spellbound::Checker;

fn main() -> Result {
  App::run(Settings::default())
}

#[derive(Default)]
struct App {
  key_input_state: widget::text_input::State,
  key: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  Encrypt(u8),
  Decrypt(u8),
  BruteForce,
  Nothing,
}

impl Application for App {
  type Executor = executor::Default;
  type Flags = ();
  type Message = Message;

  fn new(_flags: ()) -> (Self, Command<Message>) {
    (Self::default(), Command::none())
  }

  fn title(&self) -> String {
    String::new()
  }

  fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Message> {
    Column::new()
      .padding(60)
      .push(TextInput::new(
        &mut self.key_input_state,
        "",
        &self.key,
        |key| {
          let key = key.parse::<u8>().unwrap();

          if key < 25 && key > 0 {
            Message::Nothing
          } else {
            Message::Nothing
          }
        },
      ))
      .into()
  }
}
