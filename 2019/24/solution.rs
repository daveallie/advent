use std::io;
use std::collections::HashSet;

type Map = Vec<Vec<bool>>;

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> u32 {
    let mut map: Map = vec![];
    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        let row: Vec<bool> = line.chars().map(|c| c == '#').collect();

        map.push(row);
    }

    let mut seen_states: HashSet<u32> = HashSet::new();

    loop {
        let state_res = calc_res(&map);
        if seen_states.contains(&state_res) {
            return state_res;
        }
        seen_states.insert(state_res);
        iterate(&mut map);
    }
}

fn calc_res(map: &Map) -> u32 {
    map.iter().flatten().enumerate().fold(0_u32, |acc, (ind, cell)| {
        if *cell {
            acc + 2_u32.pow(ind as u32)
        } else {
            acc
        }
    })
}

fn iterate(map: &mut Map) {
    let old_map = map.clone();
    for (row_pos, row) in old_map.iter().enumerate() {
        for (col_pos, cell) in row.iter().enumerate() {
            let count = count_neighbours(&old_map, row_pos, col_pos);

            if *cell && count != 1 {
                map[row_pos][col_pos] = false;
            } else if !*cell && (count == 1 || count == 2) {
                map[row_pos][col_pos] = true;
            }
        }
    }
}

fn count_neighbours(map: &Map, row_pos: usize, col_pos: usize) -> u8 {
    let mut count = 0;

    if row_pos > 0 && map[row_pos - 1][col_pos] {
        count += 1;
    }

    if row_pos < map.len() - 1 && map[row_pos + 1][col_pos] {
        count += 1;
    }

    if col_pos > 0 && map[row_pos][col_pos - 1] {
        count += 1;
    }

    if col_pos < map[row_pos].len() - 1 && map[row_pos][col_pos + 1] {
        count += 1;
    }

    count
}

//fn render(map: &Map) {
//    for row in map {
//        for cell in row {
//            print!("{}", if *cell { '#' } else { '.' });
//        }
//        println!();
//    }
//    println!();
//}

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
