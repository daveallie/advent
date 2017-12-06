use std::io;
const RADIX: u32 = 10;

fn main() {
  let digits: Vec<u32> = read_line()
                           .unwrap()
                           .chars()
                           .filter_map(|c| c.to_digit(RADIX))
                           .collect();
  let mut total = 0;
  let mut last = 10;

  for &c in digits.iter() {
    if c == last {
      total += c;
    }
    last = c
  }

  let first = digits.first();
  let last = digits.last();

  if digits.len() > 1 && first.is_some() && last.is_some() && first.unwrap() == last.unwrap() {
    total += first.unwrap();
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
