// Trying to decrypt and encrypt a caesar cipher with a CLI
mod dictionary;
use owo_colors::OwoColorize;
use std::{fmt::Debug, process::exit, str::FromStr};

fn main() {
  exit(_main());
}

fn _main() -> i32 {
  println!("\n");

  let menu_options = ["Encrypt", "Decrypt", "Exit"];

  for (i, item) in menu_options.iter().enumerate() {
    println!("{}: {}", (i + 1).green(), item.bright_blue());
  }

  let mut menu_option = get_parsed::<usize>(&format!(
    "Choose an option ({}-{}): ",
    1.green(),
    menu_options.len().green()
  ));

  while menu_option > menu_options.len() {
    menu_option = get_parsed(&format!(
      "Choose an option ({}-{}): ",
      1.green(),
      menu_options.len().green()
    ))
  }

  let menu_option = menu_options[menu_option - 1];

  match menu_option {
    "Encrypt" => encrypt(),
    "Decrypt" => decrypt(),
    "Exit" => return 0,
    &_ => (),
  };

  0
}

fn encrypt() {
  let plain_text = get_string("What is the message that you want to encrypt? ");

  let mut shift = get_parsed(&format!(
    "What shift do you want to use ({})? ",
    "number".bright_blue()
  ));

  while shift > 26 {
    shift = get_parsed(&format!(
      "What shift do you want to use ({})? ",
      "number".bright_blue()
    ))
  }

  let encrypted = _encrypt(shift, &plain_text);

  println!("The encrypted text is: {}", encrypted.bright_blue());

  _main();
}

fn _encrypt(shift: u8, plain_text: &str) -> String {
  let mut encrypted = String::new();
  for letter in plain_text.chars() {
    if letter.is_uppercase() {
      encrypted.push_str(&(((((letter as u8 + shift) - 65) % 26) + 65) as char).to_string());
    } else if letter.is_lowercase() {
      encrypted.push_str(&(((((letter as u8 + shift) - 97) % 26) + 97) as char).to_string());
    } else {
      encrypted.push_str(&letter.to_string());
    }
  }

  encrypted
}

fn decrypt() {
  let encrypted = get_string("What is the encrypted text? ");
  let knows = get_bool(&format!(
    "Do you know the shift ({} / {})? ",
    "yes".green(),
    "no".red(),
  ));

  if knows {
    let mut shift = get_parsed(&format!("What is the shift ({})? ", "number".bright_blue()));
    while shift > 26 {
      shift = get_parsed(&format!(
        "What shift do you want to use ({})? ",
        "number".bright_blue()
      ))
    }

    let decrypted = _decrypt(shift, &encrypted);

    println!("The plain text is: {}", decrypted.bright_blue());
  } else {
    use dictionary::DICTIONARY;

    let mispelled_words_percent_threshold = 35;

    let mut tries = Vec::<Tried>::new();

    for shift in 0..26 {
      let decrypted = _decrypt(shift, &encrypted);

      let words = decrypted
        .split(' ')
        .map(|word| {
          word
            .chars()
            .filter(|_char| {
              (65..90).contains(&(*_char as u8)) || (97..122).contains(&(*_char as u8))
            })
            .collect::<String>()
        })
        .collect::<Vec<String>>();

      let percent_of_mispelled_words = {
        let mut number_of_mispelled_words = 0;

        for word in &words {
          if !DICTIONARY.contains(&&*word.to_lowercase()) {
            number_of_mispelled_words += 1;
          }
        }

        (100 * number_of_mispelled_words) / words.len()
      };

      if percent_of_mispelled_words > mispelled_words_percent_threshold {
        println!(
          "got {}, so {} is {} the shift",
          decrypted.red(),
          shift,
          "probably not".red()
        );
      } else {
        println!(
          "got {}, so {} could be the shift",
          decrypted.bright_blue(),
          shift.bright_blue()
        );
      }

      tries.push(Tried {
        percent_of_mispelled_words,
        shift,
        decrypted,
      })
    }

    let mut lowest_percent = 100;
    let mut possible_tries = Vec::<&Tried>::new();

    for tried in &tries {
      if tried.percent_of_mispelled_words < lowest_percent {
        lowest_percent = tried.percent_of_mispelled_words;
      }
    }

    for tried in &tries {
      if tried.percent_of_mispelled_words == lowest_percent {
        possible_tries.push(tried);
      }
    }

    if possible_tries.len() != 1 {
      println!(
        "The possible shifts are: {}",
        possible_tries
          .iter()
          .map(|tried| format!(
            "{} which gave {}",
            tried.shift.bright_blue(),
            tried.decrypted.bright_blue(),
          ))
          .collect::<Vec<String>>()
          .join(", ")
      );
    } else {
      let tried = possible_tries[0];

      println!(
        "The only possible shift is {} which gave {}",
        tried.shift.bright_blue(),
        tried.decrypted.bright_blue()
      )
    }
  }

  _main();
}

fn _decrypt(shift: u8, encrypted: &str) -> String {
  let mut decrypted = String::new();

  for letter in encrypted.chars() {
    if letter.is_uppercase() {
      decrypted.push_str(
        &(((((letter as u8 - shift) as i8 - 65).rem_euclid(26)) + 65) as u8 as char).to_string(),
      );
    } else if letter.is_lowercase() {
      decrypted.push_str(
        &(((((letter as u8 - shift) as i8 - 97).rem_euclid(26)) + 97) as u8 as char).to_string(),
      );
    } else {
      decrypted.push_str(&letter.to_string());
    }
  }

  decrypted
}

#[derive(Clone, Debug)]
struct Tried {
  percent_of_mispelled_words: usize,
  decrypted: String,
  shift: u8,
}

fn get_bool(prompt: &str) -> bool {
  let mut _str = get_string(prompt);

  while !_str.to_lowercase().starts_with('y') && !_str.to_lowercase().starts_with('n') {
    _str = get_string(prompt);
  }

  _str.to_lowercase().starts_with('y')
}

fn get_parsed<T>(prompt: &str) -> T
where
  T: FromStr,
  T::Err: Debug,
{
  let _str = get_string(prompt);

  let mut int = _str.parse::<T>();

  while int.is_err() {
    int = get_string(prompt).parse::<T>();
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
