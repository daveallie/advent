use std::io;

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> usize {
    let line = read_line().unwrap();
    let mut nums: Vec<usize> = line
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    if is_v2 {
        for noun in 0..100 {
            for verb in 0..100 {
                if find_first_value(&mut nums, &[noun, verb]) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        0
    } else {
        find_first_value(&mut nums, &[12, 2])
    }
}

fn find_first_value(nums: &mut Vec<usize>, replacements: &[usize; 2]) -> usize {
    let mut index = 0;
    let mut nums = nums.clone();
    nums[1] = replacements[0];
    nums[2] = replacements[1];

    loop {
        match nums[index] {
            1 | 2 => {
                let val_1 = nums[nums[index + 1]];
                let val_2 = nums[nums[index + 2]];

                let value = match nums[index] {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    _ => unreachable!(),
                };

                let replacement_index = nums[index + 3];
                nums[replacement_index] = value
            }
            99 => break,
            _ => panic!("Unknown op"),
        }

        index += 4;
    }

    nums[0]
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
