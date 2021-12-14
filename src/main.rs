// Trying to decode and encode a caesar cipher with a GUI

// #![windows_subsystem = "windows"]
// use spellbound::Checker;

const ORIGINAL_ALPHABET: [char; 52] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
  // let mut spell_checker = Checker::new();

  // println!(
  //   "{:#?}",
  //   spell_checker
  //     .check(&get_input("Type A Sentence:"))
  //     .map(|error| error.text().to_owned())
  //     .collect::<Vec<String>>()
  // );

  let menu_options = vec!["Decode", "Encode"];

  for (i, item) in menu_options.iter().enumerate() {
    println!("{}: {}", i + 1, item);
  }

  let mut menu_option = get_int(
    &format!("Choose an option (1-{}): ", menu_options.len()),
    false,
  );

  while menu_option as usize > menu_options.len() || menu_option < 0 {
    menu_option = get_int(
      &format!("Choose an option (1-{}): ", menu_options.len()),
      false,
    )
  }

  let menu_option = menu_options[menu_option as usize - 1];

  match menu_option {
    "Decode" => decode(),
    "Encode" => encode(),
    &_ => (),
  }
}

fn encode() {
  let plain_text = get_string("What is the message that you want to encode? ", false);
  let mut rotation =
    get_string("What rotation do you want to use (left / right)? ", false).to_lowercase();

  while rotation != "left" && rotation != "right" {
    rotation = get_string("What rotation do you want to use (left / right)? ", false).to_lowercase()
  }

  let mut shift = get_int("What shift do you want to use? ", false) as u8;

  while shift > 25 {
    shift = get_int("What shift do you want to use? ", false) as u8
  }

  let alphabet = create_alphabet(shift, &rotation);

  println!("{:#?}", alphabet);

  let mut encoded = plain_text;
  for (index, letter) in alphabet.iter().enumerate() {
    encoded = encoded.replace(ORIGINAL_ALPHABET[index], &letter.to_string());
  }

  println!("{}", encoded);
}

fn decode() {}

fn _input(prompt: &str) -> String {
  use io::Write;
  use std::io;

  print!("{}", prompt);

  io::stdout().flush().ok();

  let mut input = String::new();

  io::stdin().read_line(&mut input).ok();

  input.trim().to_owned()
}

fn get_string(prompt: &str, can_be_empty: bool) -> String {
  let mut input = _input(prompt);

  if !can_be_empty {
    while input == String::new() {
      input = _input(prompt)
    }
  }

  input
}

fn get_int(prompt: &str, can_be_empty: bool) -> isize {
  let _str = get_string(prompt, can_be_empty);
  let mut int = Ok(0);

  if !(can_be_empty && _str == String::new()) {
    int = _str.parse::<isize>();

    while int.is_err() {
      int = get_string(prompt, can_be_empty).parse::<isize>();
    }
  }

  int.unwrap()
}

fn create_alphabet(shift: u8, rotation: &str) -> Vec<char> {
  let use_mapper = |first_number: u8| {
    move |code: u8| {
      (((if rotation == "left" {
        code - shift
      } else {
        code + shift
      }) % 26)
        + first_number) as char
    }
  };

  let lowercase = (97..122).map(use_mapper(97)).collect::<Vec<char>>();

  let uppercase = (65..90).map(use_mapper(65)).collect::<Vec<char>>();

  [lowercase, uppercase].concat()
}
