use std::io;

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> String {
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let line = read_line().unwrap();
    let mut chars = line.chars().peekable();
    let mut layers: Vec<Vec<char>> = vec![];

    while chars.peek().is_some() {
        layers.push(chars.by_ref().take(layer_size).collect::<Vec<char>>());
    }

    if is_v2 {
        let final_image = (0..layer_size)
            .map(|index| {
                let vis_pix = layers.iter().find_map(|layer| {
                    if layer[index] != '2' {
                        Some(layer[index])
                    } else {
                        None
                    }
                });

                match vis_pix {
                    Some('1') => '0',
                    _ => ' ',
                }
            })
            .collect::<String>();

        final_image
            .as_bytes()
            .chunks(width)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("\n")
    } else {
        layers.sort_by(|l1, l2| {
            l1.iter()
                .filter(|i| **i == '0')
                .count()
                .cmp(&l2.iter().filter(|i| **i == '0').count())
        });
        let num1s = layers[0].iter().filter(|i| **i == '1').count();
        let num2s = layers[0].iter().filter(|i| **i == '2').count();
        (num1s * num2s).to_string()
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
