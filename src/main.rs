// Trying to decode and encode a caesar cipher with a GUI

// #![windows_subsystem = "windows"]
use spellbound::Checker;
use std::io::Result;

fn main() -> Result<()> {
  // let mut spell_checker = Checker::new();

  // println!(
  //   "{:#?}",
  //   spell_checker
  //     .check(&get_input("Type A Sentence:"))
  //     .map(|error| error.text().to_owned())
  //     .collect::<Vec<String>>()
  // );

  println!(
    r#" 
     _____                                  _____  _         _                  ______                         _                        _____                          _             
    /  __ \                                /  __ \(_)       | |                 |  _  \                       | |               ___    |  ___|                        | |            
    | /  \/  __ _   ___  ___   __ _  _ __  | /  \/ _  _ __  | |__    ___  _ __  | | | |  ___   ___   ___    __| |  ___  _ __   ( _ )   | |__   _ __    ___   ___    __| |  ___  _ __ 
    | |     / _` | / _ \/ __| / _` || '__| | |    | || '_ \ | '_ \  / _ \| '__| | | | | / _ \ / __| / _ \  / _` | / _ \| '__|  / _ \/\ |  __| | '_ \  / __| / _ \  / _` | / _ \| '__|
    | \__/\| (_| ||  __/\__ \| (_| || |    | \__/\| || |_) || | | ||  __/| |    | |/ / |  __/| (__ | (_) || (_| ||  __/| |    | (_>  < | |___ | | | || (__ | (_) || (_| ||  __/| |   
     \____/ \__,_| \___||___/ \__,_||_|     \____/|_|| .__/ |_| |_| \___||_|    |___/   \___| \___| \___/  \__,_| \___||_|     \___/\/ \____/ |_| |_| \___| \___/  \__,_| \___||_|   
                                                     | |                                                                                                                             
                                                     |_|                                                                                                                             "#
  );

  let menu_options = vec!["Decode", "Encode"];

  for (i, item) in menu_options.iter().enumerate() {
    println!("{}: {}", i + 1, item);
  }

  let menu_option = get_int(&format!("Choose An Option (1-{}): ", menu_options.len()));

  println!("{}", menu_option);

  Ok(())
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

fn get_int(prompt: &str) -> i128 {
  let mut int = get_string(prompt).parse::<i128>();

  while int.is_err() {
    int = get_string(prompt).parse::<i128>();
  }

  int.unwrap()
}
