use std::io;
const RADIX: u32 = 10;

fn main() {
  let digits: Vec<u32> = read_line()
                           .unwrap()
                           .chars()
                           .filter_map(|c| c.to_digit(RADIX))
                           .collect();
  let mut total = 0;
  let (first, second) = digits.split_at(digits.len() / 2);

  for (a, b) in first.iter().zip(second.iter()) {
    if a == b {
      total += 2 * a
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
