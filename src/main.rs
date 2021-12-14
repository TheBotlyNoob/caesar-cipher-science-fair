// Trying to decode and encode a caesar cipher with a GUI

// #![windows_subsystem = "windows"]
use ryaspeller::{Config, Speller};

fn main() {
  let menu_options = vec!["Decode", "Encode"];

  for (i, item) in menu_options.iter().enumerate() {
    println!("{}: {}", i + 1, item);
  }

  let mut menu_option = get_int(&format!("Choose an option (1-{}): ", menu_options.len()));

  while menu_option as usize > menu_options.len() || menu_option < 0 {
    menu_option = get_int(&format!("Choose an option (1-{}): ", menu_options.len()))
  }

  let menu_option = menu_options[menu_option as usize - 1];

  match menu_option {
    "Decode" => decode(),
    "Encode" => encode(),
    &_ => (),
  }
}

fn encode() {
  let plain_text = get_string("What is the message that you want to encode? ");

  let mut shift = get_int("What shift do you want to use (number)? ") as u8;

  while shift >= 25 {
    shift = get_int("What shift do you want to use (number)? ") as u8
  }

  let mut encoded = String::new();
  for letter in plain_text.chars() {
    if letter.is_uppercase() {
      encoded.push_str(&(((((letter as u8 + shift) - 65) % 26) + 65) as char).to_string());
    } else if letter.is_lowercase() {
      encoded.push_str(&(((((letter as u8 + shift) - 97) % 26) + 97) as char).to_string());
    } else {
      encoded.push_str(&letter.to_string());
    }
  }

  println!("{}", encoded);
}

fn decode() {
  let knows = get_bool("Do you know the shift (yes / no)? ");

  if knows {
    let shift = get_int("What is the shift (number)? ") as u8;
    let encoded = get_string("What is the encoded text? ");

    let mut decoded = String::new();
    for letter in encoded.chars() {
      if letter.is_uppercase() {
        decoded.push_str(&(((((letter as u8 - shift) - 65) % 26) + 65) as char).to_string());
      } else if letter.is_lowercase() {
        decoded.push_str(&(((((letter as u8 - shift) - 97) % 26) + 97) as char).to_string());
      } else {
        decoded.push_str(&letter.to_string());
      }
    }

    println!("{}", decoded);
  } else {
    let speller = Speller::new(Config::default());
    let spelled = speller
      .check_text("Triky Custle is a funny puzzle game.")
      .unwrap();

    println!("{:#?}", spelled);
  }
}

fn get_bool(prompt: &str) -> bool {
  let mut _str = get_string(prompt);

  while !_str.to_lowercase().starts_with("y") && !_str.to_lowercase().starts_with("n") {
    _str = get_string(prompt);
  }

  _str.to_lowercase().starts_with("y")
}

fn get_int(prompt: &str) -> isize {
  let _str = get_string(prompt);

  let mut int = _str.parse::<isize>();

  while int.is_err() {
    int = get_string(prompt).parse::<isize>();
  }

  int.unwrap()
}

fn get_string(prompt: &str) -> String {
  use io::Write;
  use std::io;

  print!("{}", prompt);

  io::stdout().flush().ok();

  let mut input = String::new();

  io::stdin().read_line(&mut input).ok();

  input.trim().to_owned()
}
