use std::io;

type Deck = Vec<u64>;

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> u64 {
    let mut instrs: Vec<String> = vec![];
    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        instrs.push(line);
    }

    let (deck_size, instr_loop_limit): (u64, u64) = if is_v2 {
        (119315717514047, 101741582076661)
    } else {
        (10007, 1)
    };

    let mut deck: Deck = (0..deck_size).collect();

    for _ in 0..instr_loop_limit {
        for instr in &instrs {
            process_line(&mut deck, &instr);
        }
    }


//    println!("{:?}", deck);
//    0
    if is_v2 {
        deck[2020]
    } else {
        deck.iter().position(|&x| x == 2019).unwrap() as u64
    }
//    deck[2019]
}

fn process_line(deck: &mut Deck, line: &str) {
    if line == "deal into new stack" {
        deck.reverse();
    } else if line.starts_with("deal with increment") {
        let inc = line.chars().skip(20).collect::<String>().parse::<usize>().unwrap();
        let len = deck.len();

        let mut new_deck: Vec<_> = deck.iter().enumerate()
            .map(|(index, x)| ((index * inc) % len, *x))
            .collect();

        new_deck.sort_by(|a, b| a.0.cmp(&b.0));
        let new_deck_iter = new_deck.into_iter().map(|(_, x)| x);

        for (old_d, new_d) in deck.iter_mut().zip(new_deck_iter) {
            *old_d = new_d
        }
    } else if line.starts_with("cut") {
        let cut = line.chars().skip(4).collect::<String>().parse::<i32>().unwrap();
        let reverse_cut = cut < 0;
        let cut = cut.abs() as usize;
        if reverse_cut {
            deck.rotate_right(cut);
        } else {
            deck.rotate_left(cut);
        }
    } else {
        panic!("No idea")
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
