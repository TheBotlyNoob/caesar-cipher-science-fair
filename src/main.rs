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
    "What shift do you want to use ({} below or equal to {})? ",
    "number".bright_blue(),
    25.green()
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

    let mut above_threshold = Vec::<AboveThreshold>::new();

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

        above_threshold.push(AboveThreshold { shift, decrypted });
      }
    }

    if above_threshold.is_empty() {
      println!("{} find a shift", "Couldn't".red());
    } else if above_threshold.len() != 1 {
      println!(
        "The possible shifts are: {}",
        above_threshold
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
      let tried = &above_threshold[0];

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
struct AboveThreshold {
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

  let mut input = input.trim().to_owned();

  if input.is_empty() {
    input = get_string(prompt);
  }

  input
}
