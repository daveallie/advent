use std::collections::{HashSet, HashMap};

struct Field {
    name: String,
    allowed_values: HashSet<usize>,
}

impl Field {
    fn from_str(input: &str) -> Self {
        let mut parts = input.split(": ");
        let name = parts.next().unwrap().to_string();
        let allowed_values = parts.next().unwrap()
            .split(" or ")
            .flat_map(|min_max| {
                let mut min_max_vals = min_max.split("-").map(|val| val.parse::<usize>().unwrap());
                min_max_vals.next().unwrap()..=min_max_vals.next().unwrap()
            })
            .collect::<HashSet<usize>>();

        Self {
            name,
            allowed_values,
        }
    }

    fn is_valid_value(&self, v: &usize) -> bool {
        self.allowed_values.contains(v)
    }
}

struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn from_str(input: &str) -> Self {
        let fields = input.split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self {
            fields
        }
    }
}

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> usize {
    let (fields, _, nearby_tickets) = parse_input(input);
    let all_allowed_values = fields.iter()
        .flat_map(|r| r.allowed_values.iter())
        .cloned()
        .collect::<HashSet<usize>>();

    nearby_tickets.iter()
        .flat_map(|ticket| ticket.fields.iter().filter(|v| !all_allowed_values.contains(v)))
        .sum::<usize>()
}

#[aoc(day16, part2)]
fn solve_part2(input: &str) -> usize {
    let (fields, your_ticket, nearby_tickets) = parse_input(input);
    let all_allowed_values = fields.iter()
        .flat_map(|r| r.allowed_values.iter())
        .cloned()
        .collect::<HashSet<usize>>();

    let field_map = fields.into_iter().map(|r| (r.name.clone(), r))
        .collect::<HashMap<String, Field>>();

    let nearby_tickets = nearby_tickets
        .into_iter()
        .filter(|t| t.fields.iter().all(|v| all_allowed_values.contains(v)))
        .collect::<Vec<Ticket>>();

    let mut allocated_fields: HashMap<String, usize> = HashMap::new();
    let mut unallocated_fields = field_map.keys().cloned().collect::<HashSet<String>>();
    let mut unallocated_cols = (0..your_ticket.fields.len()).collect::<HashSet<usize>>();

    while !unallocated_cols.is_empty() {
        for col in unallocated_cols.clone() {
            let col_values = nearby_tickets.iter()
                .map(|t| t.fields[col])
                .collect::<Vec<usize>>();

            let possible_fields = unallocated_fields.iter()
                .filter_map(|fname| field_map.get(fname))
                .filter(|field| col_values.iter().all(|v| field.is_valid_value(v)))
                .map(|field| field.name.clone())
                .collect::<Vec<String>>();

            match possible_fields.len() {
                0 => panic!(format!("Couldn't find field for col {}", col)),
                1 => {
                    allocated_fields.insert(possible_fields[0].to_string(), col);
                    unallocated_fields.remove(&possible_fields[0].to_string());
                    unallocated_cols.remove(&col);
                },
                _ => {},
            }
        }
    }

    allocated_fields.into_iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, col)| your_ticket.fields[col])
        .product()
}


fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut input_parts = input.split("\n\n");
    let fields_str = input_parts.next().unwrap();
    let your_ticket_str = input_parts.next().unwrap();
    let nearby_tickets_str = input_parts.next().unwrap();

    let fields = fields_str.lines()
        .map(|field_str| Field::from_str(field_str)).collect::<Vec<Field>>();
    let your_ticket = Ticket::from_str(your_ticket_str.lines().skip(1).next().unwrap());
    let nearby_tickets = nearby_tickets_str.lines().skip(1)
        .map(|ticket_str| Ticket::from_str(ticket_str)).collect::<Vec<Ticket>>();

    (fields, your_ticket, nearby_tickets)
}


