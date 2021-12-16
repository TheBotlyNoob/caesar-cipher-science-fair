// Trying to decrypt and encrypt a caesar cipher with a GUI

// #![windows_subsystem = "windows"]

mod dictionary;
use std::process::exit;

fn main() {
  let menu_options = vec!["Decrypt", "Encrypt", "Exit"];

  for (i, item) in menu_options.iter().enumerate() {
    println!("{}: {}", i + 1, item);
  }

  let mut menu_option = get_int(&format!("Choose an option (1-{}): ", menu_options.len()));

  while menu_option > menu_options.len() {
    menu_option = get_int(&format!("Choose an option (1-{}): ", menu_options.len()))
  }

  let menu_option = menu_options[menu_option - 1];

  match menu_option {
    "Decrypt" => decrypt(),
    "Encrypt" => encrypt(),
    "Exit" => exit(0),
    &_ => (),
  }
}

fn encrypt() {
  let plain_text = get_string("What is the message that you want to encrypt? ");

  let mut shift = get_int("What shift do you want to use (number)? ") as u8;

  while shift > 26 {
    shift = get_int("What shift do you want to use (number)? ") as u8
  }

  let encrypted = _encrypt(shift, &plain_text);

  println!("The encrypted text is: {}", encrypted);

  main();
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
  let knows = get_bool("Do you know the shift (yes / no)? ");

  if knows {
    let mut shift = get_int("What is the shift (number)? ") as u8;
    while shift > 26 {
      shift = get_int("What shift do you want to use (number)? ") as u8
    }

    let decrypted = _decrypt(shift as u8, &encrypted);

    println!("The plain text is: {}", decrypted);
  } else {
    use dictionary::DICTIONARY;

    let mispelled_words_percent_threshold = 35;

    for shift in 0..25 {
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
        println!("got {}, so {} is NOT the shift", decrypted, shift);
        continue;
      } else {
        println!("got {}, so {} is the shift", decrypted, shift);
        println!("The plain text is: {}", decrypted);
        break;
      }
    }
  }

  main();
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

fn get_bool(prompt: &str) -> bool {
  let mut _str = get_string(prompt);

  while !_str.to_lowercase().starts_with('y') && !_str.to_lowercase().starts_with('n') {
    _str = get_string(prompt);
  }

  _str.to_lowercase().starts_with('y')
}

fn get_int(prompt: &str) -> usize {
  let _str = get_string(prompt);

  let mut int = _str.parse::<usize>();

  while int.is_err() {
    int = get_string(prompt).parse::<usize>();
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
