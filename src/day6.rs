mod util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use util::*;


struct Node {
    children: HashSet<String>,
    parent: HashSet<String>,
}

impl Node {
    fn new() -> Node {
        Node { children: HashSet::new(), parent: HashSet::new() }
    }
}

type Graph = HashMap<String, Node>;


macro_rules! G {
    ($graph:expr, Add $name:expr) => {
        $graph.insert($name.to_string(), Node::new());
    };
    ($graph:expr, Insert $child:expr, Into $parent:expr) => {
        match $graph.get_mut($parent) {
            Some(node) => node.children.insert($child.to_string()),
            None => error_exit(&format!("{} don't exist", $parent)),
        };
        match $graph.get_mut($child) {
            Some(node) => node.parent.insert($parent.to_string()),
            None => error_exit(&format!("{} don't exist", $child)),
        };
    };
}

fn parse_node_list() -> Graph {
    let mut graph: Graph = Graph::new();
    for line in lines_from_stdin!() {
        let mut tokens = line.split(')').take(2);
        let child = tokens.next().unwrap();
        let parent = tokens.next().unwrap();
        if !graph.contains_key(child) {
            G!(graph, Add child);
        };
        if !graph.contains_key(parent) {
            G!(graph, Add parent);
        };
        G!(graph, Insert child, Into parent);
    }
    graph
}

fn count_path(node: &str, graph: &Graph, mem: &mut HashMap<String, usize>) -> usize {
    if mem.contains_key(node) {
        return mem[node];
    };
    let count = match graph[node].children.len() {
        0 => 0, // COM
        _ => graph[node]
            .children
            .iter()
            .map(|child| count_path(child, graph, mem) + 1)
            .sum(),
    };
    mem.insert(node.to_string(), count);
    count
}

fn total_orbits(graph: &Graph) -> usize {
    let mut mem = HashMap::new();
    let total_count = graph
        .keys()
        .map(|key| count_path(key, graph, &mut mem))
        .sum();
    mem.iter()
        .for_each(|(key, value)| println!("{} : {}", key, value));
    total_count
}

fn shortest_path(graph: &Graph, start: &str, end: &str) -> usize {
    let mut distances: HashMap<String, usize> = HashMap::new();
    for key in graph.keys() {
        distances.insert(
            key.to_string(),
            if key == start { 0 } else { std::usize::MAX },
        );
    }

    let mut visited : HashSet<String> = HashSet::new();
    visited.insert(start.to_string());

    let mut queue : VecDeque<&str> = VecDeque::new();
    queue.push_back(start);

    while queue.len() > 0 {
        let node_name = queue.pop_front().unwrap();
        eprintln!("Visiting {}", node_name);

        let node = &graph[node_name];
        let distance = distances[node_name] + 1;
        for neighbor in node.children
            .iter()
            .chain(node.parent.iter())
            .filter(|&node_name| !visited.contains(node_name))
        {
            let d = distances.get_mut(neighbor).unwrap();
            *d = (*d).min(distance);
            queue.push_back(neighbor);
        };
        visited.insert(node_name.to_string());
    }
    distances[end]
}

fn main() {
    let graph = parse_node_list();
    match part_id_from_cli() {
        PartID::One => println!("{}", total_orbits(&graph)),
        PartID::Two => {
            let start = graph["YOU"].children.iter().next().unwrap();
            let end = graph["SAN"].children.iter().next().unwrap();
            println!("{}", shortest_path(&graph, start, end));
        },
    };
}
