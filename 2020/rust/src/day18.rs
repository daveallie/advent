#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    input.lines().map(|line| solve(line.replace(" ", ""), false)).sum()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> usize {
    input.lines().map(|line| solve(line.replace(" ", ""), true)).sum()
}

fn solve(expression: String, part2: bool) -> usize {
    if expression.contains("(") {
        let index = expression.find("(").unwrap();
        let mut b_count = 0;
        for (i, c) in expression.chars().enumerate().skip(index) {
            if c == ')' {
                if b_count == 1 {
                    let inner_value = solve(expression[index + 1..i].to_string(), part2);
                    let subbed = format!("{}{}{}", &expression[0..index], inner_value, &expression[i + 1..expression.len()]);
                    return solve(subbed, part2)
                }

                b_count -= 1;
            } else if c == '(' {
                b_count += 1;
            }
        }
    }

    if part2 && expression.contains("+") && expression.contains("*") {
        let index = expression.find("*").unwrap();
        let left_value = solve(expression[0..index].to_string(), part2);
        let right_value = solve(expression[index + 1..expression.len()].to_string(), part2);
        return left_value * right_value;
    }

    // no parens
    let mut total = 0_usize;
    let mut val = 0_usize;
    let mut is_add = true;
    for c in expression.chars() {
        match c {
            '0'..='9' => val = (val * 10) + (c as usize - '0' as usize),
            '+' | '*' => {
                if is_add {
                    total += val;
                } else {
                    total *= val;
                }
                val = 0;
                is_add = c == '+';
            },
            _ => panic!("Unknown char")
        }
    }

    if is_add {
        total += val;
    } else {
        total *= val;
    }

    total
}
