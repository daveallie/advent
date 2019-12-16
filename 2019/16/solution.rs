use std::io;

enum TransformMapValue {
    PosOne,
    PosZero,
    NegOne,
    NegZero,
}

impl TransformMapValue {
    fn next(&self) -> Self {
        match self {
            Self::PosOne => Self::PosZero,
            Self::PosZero => Self::NegOne,
            Self::NegOne => Self::NegZero,
            Self::NegZero => Self::PosOne,
        }
    }

    fn to_i8(&self) -> i8 {
        match self {
            Self::PosOne => 1,
            Self::NegOne => -1,
            Self::PosZero | Self::NegZero => 0,
        }
    }
}

struct TransformMapIter {
    total_remaining: usize,
    repeat: usize,
    curr: TransformMapValue,
    curr_remaining: usize,
}

impl TransformMapIter {
    fn new(length: usize, dig: usize) -> Self {
        Self {
            total_remaining: length,
            repeat: dig + 1,
            curr: TransformMapValue::NegZero,
            curr_remaining: dig,
        }
    }
}

impl Iterator for TransformMapIter {
    type Item = i8;

    fn next(&mut self) -> Option<i8> {
        if self.total_remaining == 0 {
            return None;
        }

        if self.curr_remaining == 0 {
            self.curr = self.curr.next();
            self.curr_remaining = self.repeat;
        }

        self.curr_remaining -= 1;
        self.total_remaining -= 1;
        Some(self.curr.to_i8())
    }
}

fn transform(digs: &Vec<i8>) -> Vec<i8> {
    (0..digs.len())
        .map(|n_dig| {
            let new_val = digs
                .iter()
                .zip(TransformMapIter::new(digs.len(), n_dig))
                .map(|(dig, multi)| (dig * multi) as i32)
                .sum::<i32>();
            (new_val.abs() % 10) as i8
        })
        .collect()
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> String {
    let line = read_line().unwrap();
    let mut digs: Vec<i8> = line.bytes().map(|b| (b - b'0') as i8).collect();

    if is_v2 {
        let offset: usize = digs
            .iter()
            .take(7)
            .enumerate()
            .map(|(i, dig)| (*dig as usize) * 10_usize.pow((6 - i) as u32))
            .sum();
        if offset < digs.len() * 10000 / 2 {
            panic!("Shortcut where transform map is all 1s can't be taken");
        }

        // only need to consider digs after offset as each digit is calculated using itself and future digits only
        let full_length = digs.len() * 10_000;
        digs = digs
            .into_iter()
            .cycle()
            .take(full_length)
            .skip(offset)
            .collect();

        // transform map looks like
        // 1 1 1 1 ...
        // 0 1 1 1 ...
        // 0 0 1 1 ...
        // 0 0 0 1 ...
        // we can just reverse the array to get
        // 1 0 0 0 ...
        // 1 1 0 0 ...
        // 1 1 1 0 ...
        // 1 1 1 1 ...
        // which just means each new digit is the previous new digit plus the old digit
        // effectively a cumsum
        digs.reverse();

        for _ in 0..100 {
            for i in 1..digs.len() {
                digs[i] = (digs[i] + digs[i - 1]) % 10;
            }
        }

        digs.reverse();
    } else {
        for _ in 0..100 {
            digs = transform(&digs);
        }
    }

    digs.iter()
        .take(8)
        .map(|i| i.to_string())
        .collect::<String>()
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
