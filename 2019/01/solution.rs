use std::io;

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> u32 {
    let mut total: u32 = 0;

    loop {
        let line = read_line().unwrap();

        if line == "" {
            break;
        }

        let mass: u32 = line.parse().unwrap();
        let new_fuel = get_module_fuel(mass);
        total += new_fuel;

        if is_v2 {
            let mut fuel_to_fuel = new_fuel;

            while fuel_to_fuel > 8 {
                let fuel_fuel = get_module_fuel(fuel_to_fuel);
                total += fuel_fuel;
                fuel_to_fuel = fuel_fuel;
            }
        }
    }

    return total;
}

fn get_module_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
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
