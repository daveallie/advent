use std::io;

#[derive(Debug)]
struct Password {
    val: Vec<u8>,
}

impl Password {
    fn from(val: Vec<u8>) -> Self {
        Self { val }
    }

    fn build_next(&self) -> Self {
        Self::from(Self::get_next_num(&self.val))
    }

    fn is_larger(&self, b: &Password) -> bool {
        let mut larger = false;
        for (ai, bi) in self.val.iter().zip(b.val.iter()) {
            if ai == bi {
                continue;
            }

            if ai > bi {
                larger = true;
            }
            break;
        }
        larger
    }

    fn is_valid(&self, is_v2: bool) -> bool {
        let invalid_nums = if is_v2 {
            self.val[0..4]
                .iter()
                .enumerate()
                .filter(|(index, val)| **val == self.val[index + 1] && **val == self.val[index + 2])
                .map(|(_, val)| val.to_owned())
                .collect::<Vec<u8>>()
        } else {
            vec![]
        };

        self.val[0..5]
            .iter()
            .enumerate()
            .any(|(index, val)| !invalid_nums.contains(val) && *val == self.val[index + 1])
    }

    fn get_next_num(parts: &Vec<u8>) -> Vec<u8> {
        let mut parts = parts.clone();

        if parts.len() == 1 {
            parts[0] += 1;
            return parts;
        }

        let last_ind = parts.len() - 1;

        if parts[last_ind] < 9 {
            parts[last_ind] += 1;
            parts
        } else {
            let mut parts = Self::get_next_num(&parts[0..last_ind].to_vec());
            parts.push(parts[last_ind - 1]);
            parts
        }
    }
}

struct PasswordIter {
    curr: Password,
    max: Password,
    is_v2: bool,
}

impl Iterator for PasswordIter {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let mut new_next = self.curr.build_next();

        while !new_next.is_valid(self.is_v2) {
            new_next = new_next.build_next();
        }

        if new_next.is_larger(&self.max) {
            return None;
        }

        self.curr = new_next;
        Some(())
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> usize {
    let line = read_line().unwrap();
    let mut input_bounds = line
        .split("-")
        .map(|i| {
            i.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .map(|parts| Password::from(parts));

    let min = input_bounds.next().unwrap();
    let max = input_bounds.next().unwrap();

    let password_iter = PasswordIter {
        curr: set_new_min(&min),
        max,
        is_v2,
    };

    password_iter.count()
}

fn set_new_min(password: &Password) -> Password {
    let mut curr_max = password.val[0];
    let mut new_min: Vec<u8> = password
        .val
        .iter()
        .map(|i| {
            if i > &curr_max {
                curr_max = i.clone();
            }
            curr_max
        })
        .collect();

    new_min[5] -= 1;
    Password::from(new_min)
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
