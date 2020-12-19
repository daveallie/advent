use std::collections::HashMap;

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> u64 {
    let mut inv_mask = 0_u64;
    let mut base = 0_u64;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            inv_mask = 0;
            base = 0;

            for (i, c) in line[7..line.len()].chars().rev().enumerate() {
                match c {
                    'X' => inv_mask += 1 << i,
                    '0' => {},
                    '1' => base += 1 << i,
                    _ => panic!(),
                }
            }
        } else {
            let mut parts = line.split(" = ");
            let addr_str = parts.next().unwrap();
            let addr = addr_str[4..addr_str.len() - 1].parse::<u64>().unwrap();

            let value = parts.next().unwrap().parse::<u64>().unwrap();
            let adj_value = (inv_mask & value) | base;

            memory.insert(addr, adj_value);
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut mask = String::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            mask = line[7..line.len()].to_string();
        } else {
            let mut parts = line.split(" = ");
            let addr_str = parts.next().unwrap();
            let addr = addr_str[4..addr_str.len() - 1].parse::<u64>().unwrap();
            let mask_addresses = build_mask_addresses(&mask, addr);

            let value = parts.next().unwrap().parse::<u64>().unwrap();

            for mask_addr in mask_addresses {
                memory.insert(mask_addr, value);
            }
        }
    }

    memory.values().sum()
}

fn build_mask_addresses(mask: &str, addr: u64) -> Vec<u64> {
    let mut floating_bits = vec![];
    let mut base_addr = 0_u64;

    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            'X' => floating_bits.push(i),
            '1' => base_addr += 1 << i,
            '0' => if (addr >> i) & 1 > 0 { base_addr += 1 << i },
            _ => panic!(),
        }
    }

    (0..2_usize.pow(floating_bits.len() as u32)).map(|addr_ind| {
        let mut addr = base_addr.clone();
        for (i, c) in format!("{:b}", addr_ind).chars().rev().enumerate() {
            if c == '0' {
                continue
            }

            addr += 1 << floating_bits[i]
        }
        addr
    }).collect()
}
