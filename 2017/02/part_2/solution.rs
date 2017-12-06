use std::io;

fn main() {
  let mut total = 0;

  loop {
    let line = read_line().unwrap();

    if line == "" {
      break;
    }

    let mut found = false;

    let mut numbers: Vec<u32> = line.split("\t").filter_map(|num_str| {
      match num_str.parse::<u32>() {
        Ok(num) => Some(num),
        _ => None
      }
    }).collect();
    numbers.sort();

    for i in 1..numbers.len() {
      for j in 0..i {
        if numbers[i] % numbers[j] == 0 {
          total += numbers[i] / numbers[j];
          found = true;
          break;
        }
      }
      if found {
        break;
      }
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
