#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let target = lines.next().unwrap().parse::<usize>().unwrap();
    lines.next().unwrap()
        .split(",")
        .filter(|b| b != &"x")
        .map(|b| b.parse::<usize>().unwrap())
        .map(|b| (b, b - target % b))
        .min_by(|(_, wait1), (_, wait2)| wait1.cmp(wait2))
        .map(|(b, wait)| b * wait)
        .unwrap()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    lines.next();

    let data = lines.next().unwrap()
        .split(",")
        .map(|b| b.parse::<usize>().unwrap_or(0))
        .enumerate()
        .filter(|&(_, b)| b != 0)
        .map(|(i, b)| {
            if i == 0 {
                return (b, 0)
            }
            let residue = (lcm(b, i) - i) % b;
            (b, residue)
        })
        .collect::<Vec<(usize, usize)>>();

    crt(&data)
}

fn crt(data: &[(usize, usize)]) -> usize {
    let product = data.iter().map(|(b, _)| b).product::<usize>();

    data.iter().map(|&(b, r)| {
        let p = product / b;
        r * mod_inverse(p, b) * p
    }).sum::<usize>() % product
}

fn mod_inverse(x: usize, n: usize) -> usize {
    let x = egcd(x, n).1;
    (x % n as isize + n as isize) as usize % n
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / egcd(a, b).0
}

fn egcd(a: usize, b: usize) -> (usize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) as isize * x, x)
    }
}
