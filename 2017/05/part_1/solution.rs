use std::io;

fn main() {
    let mut nums: Vec<i32> = vec![];
    let mut jumps = 0;
    let mut index: i32 = 0;

    loop {
        let line = read_line().unwrap();

        if line == "" {
            break;
        }

        if let Ok(num) = line.parse::<i32>() {
            nums.push(num);
        }
    }

    let vector_len = nums.len() as i32;

    while 0 <= index && index < vector_len {
        let old_index = index as usize;
        index += nums[old_index];
        nums[old_index] += 1;
        jumps += 1;
    }

    println!("{}", jumps);
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
