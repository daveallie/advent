use std::io;
use std::io::Read;

fn main() {
  let mut buffer = [0u8; 256];
  let mut should_set_first = true;
  let mut first = None;
  let mut last = None;
  let mut prev = 10;
  let mut total = 0u32;
  let mut len = 0;
  let mut done = false;

  while let Ok(n) = io::stdin().read(&mut buffer) {
    if n == 0 {
      break;
    }

    len += n;

    if should_set_first {
      should_set_first = false;
      first = byte_to_num(buffer[0]);
    }

    for i in buffer[..n].iter().map(|i| byte_to_num(*i)) {
      match i {
        Some(digit) => {
          if digit == prev {
            total += digit as u32;
          }
          prev = digit;
          last = Some(digit);
        },
        None => {
          done = true;
          break;
        }
      }
    }

    if done {
      break;
    }
  }

  if len > 1 && first.is_some() && last.is_some() && first.unwrap() == last.unwrap() {
    total += first.unwrap() as u32;
  }

  println!("{}", total);
}

fn byte_to_num(byte: u8) -> Option<u8> {
  if 48 <= byte && byte <= 57 {
    Some(byte - 48)
  } else {
    None
  }
}
