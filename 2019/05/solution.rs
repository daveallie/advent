use std::io;

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> i32 {
    let line = read_line().unwrap();
    let mut nums: Vec<i32> = line.split(",").map(|i| i.parse::<i32>().unwrap()).collect();

    let mut index: usize = 0;
    let mut last_output = 0;

    loop {
        let inst = nums[index];
        let op = inst % 100;
        let modes: [bool; 2] = [(inst / 100) % 2 == 1, (inst / 1000) % 2 == 1];

        match op {
            1 | 2 | 7 | 8 => {
                let val_1 = get_val(&nums, modes[0], nums[index + 1]);
                let val_2 = get_val(&nums, modes[1], nums[index + 2]);

                let value = match op {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    7 if val_1 < val_2 => 1,
                    7 => 0,
                    8 if val_1 == val_2 => 1,
                    8 => 0,
                    _ => unreachable!(),
                };

                let addr = nums[index + 3] as usize;
                nums[addr] = value;
                index += 4;
            }
            3 => {
                let addr = nums[index + 1] as usize;
                nums[addr] = if is_v2 { 5 } else { 1 };
                index += 2;
            }
            4 => {
                let val_1 = get_val(&nums, modes[0], nums[index + 1]);
                last_output = val_1;
                index += 2
            }
            5 | 6 => {
                let val_1 = get_val(&nums, modes[0], nums[index + 1]);
                let val_2 = get_val(&nums, modes[1], nums[index + 2]);

                if op == 5 && val_1 != 0 || op == 6 && val_1 == 0 {
                    index = val_2 as usize;
                } else {
                    index += 3;
                }
            }
            99 => break,
            _ => panic!("Unknown op: {}", op),
        }
    }
    last_output
}

fn get_val(nums: &Vec<i32>, immediate_mode: bool, val: i32) -> i32 {
    if immediate_mode {
        val
    } else {
        nums[val as usize]
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            Ok(input)
        }
        Err(error) => Err(error),
    }
}
