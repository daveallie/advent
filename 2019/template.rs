use std::io;

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) {
    loop {
        let line = read_line().unwrap();

        if line == "" {
            break;
        }
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
