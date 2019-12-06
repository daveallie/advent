use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Node {
    depth: usize,
    children: Vec<String>,
    parent: Option<String>,
}

impl Node {
    fn new() -> Self {
        Node {
            depth: 0,
            children: vec![],
            parent: None,
        }
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> usize {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    loop {
        let line = read_line().unwrap();

        if line == "" {
            break;
        }

        let mut parts = line.split(")");
        let from_node_name = parts.next().unwrap().to_string();
        let to_node_name = parts.next().unwrap().to_string();

        if !nodes.contains_key(&from_node_name) {
            nodes.insert(from_node_name.clone(), Node::new());
        }
        if !nodes.contains_key(&to_node_name) {
            nodes.insert(to_node_name.clone(), Node::new());
        }

        let from_node = nodes.get_mut(&from_node_name).unwrap();
        from_node.children.push(to_node_name.clone());

        let mut to_node = nodes.get_mut(&to_node_name).unwrap();
        to_node.parent = Some(from_node_name)
    }

    build_depths(&mut nodes, "COM", 0);

    if is_v2 {
        find_jumps_between(&nodes, "YOU", "SAN")
    } else {
        nodes.values().map(|node| node.depth).sum()
    }
}

fn build_depths(nodes: &mut HashMap<String, Node>, node_name: &str, depth: usize) {
    {
        let mut node = nodes.get_mut(node_name).unwrap();
        node.depth = depth;
    }

    nodes[node_name]
        .children
        .clone()
        .iter()
        .for_each(|child_name| build_depths(nodes, child_name, depth + 1));
}

fn find_jumps_between(
    nodes: &HashMap<String, Node>,
    node_1_target: &str,
    node_2_target: &str,
) -> usize {
    let mut node_1_target = node_1_target.to_string();
    let mut node_2_target = node_2_target.to_string();
    let mut jumps = 0;

    while nodes[&node_1_target].parent != nodes[&node_2_target].parent {
        let node_1_parent = nodes[&node_1_target].parent.as_ref();
        let node_2_parent = nodes[&node_2_target].parent.as_ref();

        if nodes[&node_2_target].depth < nodes[&node_1_target].depth {
            node_1_target = node_1_parent.unwrap().to_string();
            jumps += 1;
        } else if nodes[&node_2_target].depth > nodes[&node_1_target].depth {
            node_2_target = node_2_parent.unwrap().to_string();
            jumps += 1;
        } else {
            node_1_target = node_1_parent.unwrap().to_string();
            node_2_target = node_2_parent.unwrap().to_string();
            jumps += 2;
        }
    }

    jumps
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
