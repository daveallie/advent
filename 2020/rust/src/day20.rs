use std::collections::{HashMap, HashSet};

enum Rule {
    Str(String),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn and(&self, other: &Self) -> Self {
        match self {
            Self::Str(a) => {
                match other {
                    Self::Str(b) => Self::Str(format!("{}{}", a, b)),
                    Self::Or(b1, b2) => Self::Or(Box::from(self.and(b1)), Box::from(self.and(b2))),
                }
            },
            Self::Or(a1, a2) => Self::Or(Box::from(a1.and(other)), Box::from(a2.and(other))),
        }
    }

    fn unwind(self) -> Vec<String> {
        match self {
            Self::Str(s) => vec![s],
            Self::Or(r1, r2) => r1.unwind().into_iter().chain(r2.unwind().into_iter()).collect(),
        }
    }
}

impl ToString for Rule {
    fn to_string(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            Self::Or(r1, r2) => format!("({} | {})", r1.to_string(), r2.to_string()),
        }
    }
}

#[aoc(day19, part1)]
fn solve_part1(input: &str) -> usize {
    let mut input_parts = input.split("\n\n");
    let rule_0 = parse_and_resolve_rules(input_parts.next().unwrap());
    let possible_values = rule_0.unwind().into_iter().collect::<HashSet<String>>();

    input_parts.next().unwrap().lines()
        .filter(|line| possible_values.contains(&line.to_string()))
        .count()
}

fn parse_and_resolve_rules(input: &str) -> Rule {
    let raw_rules = input.lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let rule_id = parts.next().unwrap().parse::<usize>().unwrap();
            let rule_str = parts.next().unwrap().to_string();

            (rule_id, rule_str)
        }).collect::<HashMap<usize, String>>();

    let mut rules: HashMap<usize, Rule> = HashMap::new();
    resolve_rule(raw_rules.get(&0).unwrap(), &mut rules, &raw_rules)
}

fn resolve_rule(rule_str: &str, rules: &mut HashMap<usize, Rule>, raw_rules: &HashMap<usize, String>) -> Rule {
    if rule_str.contains(" | ") {
        let mut rule_parts = rule_str.split(" | ");
        let first_rule = resolve_rule(rule_parts.next().unwrap(), rules, raw_rules);
        let second_rule = resolve_rule(rule_parts.next().unwrap(), rules, raw_rules);
        return Rule::Or(Box::from(first_rule), Box::from(second_rule));
    }

    let mut rule = Rule::Str(String::new());
    for token in rule_str.split_whitespace() {
        if token.contains("\"") {
            rule = rule.and(&Rule::Str(String::from(&token[1..2])));
        } else {
            let nested_rule_id = token.parse::<usize>().unwrap();

            if !rules.contains_key(&nested_rule_id) {
                let nested_raw_rule = raw_rules.get(&nested_rule_id).unwrap();
                let nested_rule = resolve_rule(nested_raw_rule, rules, raw_rules);
                rules.insert(nested_rule_id, nested_rule);
            }
            let nested_rule = rules.get(&nested_rule_id).unwrap();
            rule = rule.and(nested_rule);
        }
    }

    rule
}
