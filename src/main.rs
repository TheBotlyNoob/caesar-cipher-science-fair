// Trying to decode and encode a caesar cipher with a GUI

// #![windows_subsystem = "windows"]
mod dictionary;

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
  let encoded = get_string("What is the encoded text? ");

  if knows {
    let shift = get_int("What is the shift (number)? ") as u8;

    let decoded = _decode(shift, &encoded);

    println!("{}", decoded);
  } else {
    use dictionary::DICTIONARY;
    let mispelled_words_threshold = 25;

    let mut decoded = String::new();

    for shift in 0..25 {
      decoded = _decode(shift, &encoded);
      let words = decoded.split(' ').collect::<Vec<&str>>();

      let percent_of_mispelled_words = {
        let mut number_of_mispelled_words = 0;
        for word in &words {
          if !DICTIONARY.contains(word) {
            number_of_mispelled_words += 1;
          }
        }

        (100 * number_of_mispelled_words) / words.len()
      };

      if percent_of_mispelled_words < mispelled_words_threshold {
        break;
      } else {
        continue;
      };
    }

    println!("{}", decoded);
  }
}

fn _decode(shift: u8, encoded: &str) -> String {
  let mut decoded = String::new();
  for letter in encoded.chars() {
    if letter.is_uppercase() {
      decoded.push_str(&(((((letter as u8 - shift) - 65) % 26) + 65) as char).to_string());
    } else if letter.is_lowercase() {
      println!("{}", (letter as u8 - shift));
      decoded.push_str(&(((((letter as u8 - shift) - 97) % 26) + 97) as char).to_string());
    } else {
      decoded.push_str(&letter.to_string());
    }
  }

  decoded
}

fn get_bool(prompt: &str) -> bool {
  let mut _str = get_string(prompt);

  while !_str.to_lowercase().starts_with('y') && !_str.to_lowercase().starts_with('n') {
    _str = get_string(prompt);
  }

  _str.to_lowercase().starts_with('y')
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
