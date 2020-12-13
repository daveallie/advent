use std::str::FromStr;
use std::collections::HashMap;
use std::ops::{Not, BitAnd, BitOr};
use crate::day07::WireOrValue::{Wire, Value};

#[derive(Clone, Debug)]
enum WireOrValue {
    Wire(String),
    Value(u16)
}

impl FromStr for WireOrValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(v) => Ok(Self::Value(v)),
            Err(_) => Ok(Self::Wire(s.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Not(WireOrValue),
    Direct(WireOrValue),
    And(WireOrValue, WireOrValue),
    Or(WireOrValue, WireOrValue),
    LShift(WireOrValue, WireOrValue),
    RShift(WireOrValue, WireOrValue),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(op_str: &str) -> Result<Self, Self::Err> {
        let mut op_parts = op_str.split_whitespace();

        if op_str.starts_with("NOT") {
            let w = WireOrValue::from_str(op_parts.skip(1).next().unwrap()).unwrap();
            return Ok(Operation::Not(w))
        }

        if op_str.contains("AND") || op_str.contains("OR") || op_str.contains("SHIFT") {
            let w1 = WireOrValue::from_str(op_parts.next().unwrap()).unwrap();
            let op = op_parts.next().unwrap();
            let w2 = WireOrValue::from_str(op_parts.next().unwrap()).unwrap();

            match op {
                "AND" => Ok(Operation::And(w1, w2)),
                "OR" => Ok(Operation::Or(w1, w2)),
                "LSHIFT" => Ok(Operation::LShift(w1, w2)),
                "RSHIFT" => Ok(Operation::RShift(w1, w2)),
                _ => Err(format!("Unknown operation: {}", op)),
            }
        } else {
            Ok(Operation::Direct(WireOrValue::from_str(op_str).unwrap()))
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> HashMap<String, Operation> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let op_str = parts.next().unwrap();
            let wire = parts.next().unwrap().to_string();

            (wire, Operation::from_str(op_str).unwrap())
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(wire_map: &HashMap<String, Operation>) -> u16 {
    let mut wire_values: HashMap<String, u16> = HashMap::new();
    calculate_wire_value(wire_map, &mut wire_values, &Wire("a".to_string()))
}

#[aoc(day7, part2)]
fn solve_part2(wire_map: &HashMap<String, Operation>) -> u16 {
    let mut wire_values: HashMap<String, u16> = HashMap::new();
    let a_value = calculate_wire_value(wire_map, &mut wire_values, &Wire("a".to_string()));

    wire_values.clear();
    let mut new_wire_map = wire_map.clone();
    new_wire_map.insert("b".to_string(), Operation::Direct(Value(a_value)));
    calculate_wire_value(&new_wire_map, &mut wire_values, &Wire("a".to_string()))
}

fn calculate_wire_value(wire_map: &HashMap<String, Operation>, wire_values: &mut HashMap<String, u16>, w: &WireOrValue) -> u16 {
    let wire = match w {
        Wire(wire) => wire,
        WireOrValue::Value(v) => return *v,
    };
    if wire_values.contains_key(wire) {
        return wire_values[wire];
    }

    let value = match wire_map.get(wire) {
        Some(Operation::Direct(w)) => calculate_wire_value(wire_map, wire_values, w),
        Some(Operation::Not(w)) => calculate_wire_value(wire_map, wire_values, w).not(),
        Some(Operation::And(w1, w2)) => {
            let v1 = calculate_wire_value(wire_map, wire_values, w1);
            let v2 = calculate_wire_value(wire_map, wire_values, w2);
            v1.bitand(v2)
        }
        Some(Operation::Or(w1, w2)) => {
            let v1 = calculate_wire_value(wire_map, wire_values, w1);
            let v2 = calculate_wire_value(wire_map, wire_values, w2);
            v1.bitor(v2)
        }
        Some(Operation::LShift(w1, w2)) => {
            let v1 = calculate_wire_value(wire_map, wire_values, w1);
            let v2 = calculate_wire_value(wire_map, wire_values, w2);
            v1 << v2
        },
        Some(Operation::RShift(w1, w2)) => {
            let v1 = calculate_wire_value(wire_map, wire_values, w1);
            let v2 = calculate_wire_value(wire_map, wire_values, w2);
            v1 >> v2
        },
        None => panic!(format!("Unknown wire {}", wire)),
    };

    wire_values.insert(wire.to_string(), value);
    value
}
