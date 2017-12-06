use std::num;
use std::io;

fn main() {
  let num = read_line_as_num().unwrap().unwrap();
  let floored_sqrt = (num as f64).sqrt() as i32;
  
  if floored_sqrt * floored_sqrt == num {
    println!("{}", floored_sqrt - 1);
  } else {
    let next_odd_sqrt = floored_sqrt + 1 + floored_sqrt % 2;
    let distance_to_next_odd_square = next_odd_sqrt * next_odd_sqrt - num;
    let distance_to_next_corner = distance_to_next_odd_square % (next_odd_sqrt - 1);
    let corner_to_center = (next_odd_sqrt - 1) / 2;
    let distance_to_side_center = (distance_to_next_corner - corner_to_center).abs();
    println!("{}", corner_to_center + distance_to_side_center);
  }
}

fn read_line_as_num() -> Result<Result<i32, num::ParseIntError>, io::Error> {
  let mut input = String::new();

  if let Err(error) = io::stdin().read_line(&mut input) {
    return Err(error);
  }

  input.pop();
  Ok(input.parse::<i32>())
}
