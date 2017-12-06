use std::io;

fn main() {
  let mut total = 0;

  loop {
    let line = read_line().unwrap();

    if line == "" {
      break;
    }

    if is_passcode_valid(&line) {
      total += 1;
    }
  }

  println!("{}", total);
}

fn read_line() -> Result<String, io::Error> {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
      Ok(_) => {
        input.pop();
        Ok(input)
      },
      Err(error) => Err(error),
  }
}

fn is_passcode_valid(line: &str) -> bool {
  let mut words: Vec<String> = line.split(" ").map(|word| {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
  }).collect();
  let original_length = words.len();
  words.sort();
  words.dedup();

  original_length == words.len()
}
