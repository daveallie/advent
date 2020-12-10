use std::collections::HashMap;

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    input.split("\n\n").filter(|passport| {
        passport.contains("ecl:") && passport.contains("pid:") &&
            passport.contains("eyr:") && passport.contains("hcl:") &&
            passport.contains("byr:") && passport.contains("iyr:") &&
            passport.contains("hgt:")
    }).count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    input.split("\n\n").filter(|raw_passport| {
        let tokens = raw_passport.split_whitespace().flat_map(|x| x.split(':'));
        let passport: HashMap<&str, &str> = tokens.clone().step_by(2)
            .zip(tokens.skip(1).step_by(2))
            .collect();

        validate_year(&passport.get("byr"), 1920, 2002) &&
            validate_year(&passport.get("iyr"), 2010, 2020) &&
            validate_year(&passport.get("eyr"), 2020, 2030) &&
            validate_height(&passport.get("hgt")) &&
            validate_hair_color(&passport.get("hcl")) &&
            validate_eye_color(&passport.get("ecl")) &&
            validate_passport_id(&passport.get("pid"))
    }).count()
}

fn validate_year(value_opt: &Option<&&str>, min: usize, max: usize) -> bool {
    validate_number(value_opt, min, max)
}

fn validate_height(value_opt: &Option<&&str>) -> bool {
    value_opt.map(|v| {
        if v.ends_with("cm") {
            validate_number(&Some(&&v[0..v.len() - 2]), 150, 193)
        } else if v.ends_with("in") {
            validate_number(&Some(&&v[0..v.len() - 2]), 59, 76)
        } else {
            false
        }
    }).unwrap_or(false)
}

fn validate_hair_color(value_opt: &Option<&&str>) -> bool {
    value_opt
        .filter(|value| value.len() == 7 && value.starts_with("#") &&
            value.chars().skip(1)
                .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c)))
        .is_some()
}

fn validate_eye_color(value_opt: &Option<&&str>) -> bool {
    value_opt
        .filter(|value| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value))
        .is_some()
}

fn validate_passport_id(value_opt: &Option<&&str>) -> bool {
    value_opt
        .filter(|value| value.len() == 9 && value.chars().all(|v| v.is_ascii_digit()))
        .is_some()
}

fn validate_number(value_opt: &Option<&&str>, min: usize, max: usize) -> bool {
    value_opt
        .map(|value| value.parse::<usize>().map_or(None, |v| Some(v)))
        .flatten()
        .map(|value| min <= value && value <= max)
        .unwrap_or(false)
}
