extern crate regex;

mod util;

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use util::error_exit;

struct Graph {
    node_list: HashMap<String, usize>,
    adj_list: Vec<Vec<(usize, usize)>>,
    units: Vec<usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            node_list: HashMap::new(),
            adj_list: Vec::new(),
            units: Vec::new(),
        }
    }

    fn get_node(&mut self, name: &str) -> usize {
        if !self.node_list.contains_key(name) {
            let count = self.count();
            self.node_list.insert(String::from(name), count);
            self.adj_list.push(Vec::new());
            self.units.push(0);
        }
        self.node_list[name]
    }

    fn count(&self) -> usize {
        self.node_list.len()
    }
}

fn _visit(node: usize, sorted: &mut Vec<usize>, visited: &mut Vec<bool>, graph: &Graph) {
    if visited[node] {
        return;
    }
    for &(child, _) in graph.adj_list[node].iter() {
        _visit(child, sorted, visited, graph);
    }
    visited[node] = true;
    sorted.push(node);
}

fn topological_sort(graph: &Graph) -> Vec<usize> {
    let mut visited = [false].repeat(graph.count());
    let mut sorted = Vec::<usize>::new();
    for node in 0..graph.count() {
        _visit(node, &mut sorted, &mut visited, graph);
    }
    sorted.reverse();
    sorted
}

fn min_multiple(base: usize, min_val: usize) -> usize {
    (min_val - 1) / base + 1
}

fn required_ore(fuel: usize, sorted_id: &Vec<usize>, graph: &Graph) -> usize {
    let mut counts = [0usize].repeat(graph.count());
    counts[graph.node_list["FUEL"]] = fuel;

    for &node in sorted_id[0..sorted_id.len() - 1].iter() {
        let min_count = counts[node];
        let multiple = min_multiple(graph.units[node], min_count);

        for &(child_id, child_coef) in graph.adj_list[node].iter() {
            counts[child_id] += multiple * child_coef;
        }
    }

    counts[graph.node_list["ORE"]]
}

fn main() {
    let matcher = Regex::new(r"(\d+) ([A-Z]+)").expect("Failed to build regex");
    let mut graph = Graph::new();

    for line in lines_from_stdin!() {
        let components: Vec<(usize, usize)> = matcher
            .captures_iter(&line)
            .map(|cap| (graph.get_node(&cap[2]), cap[1].parse::<usize>().expect("_")))
            .collect();
        match components.split_last() {
            Some((&(rhs_id, rhs_coef), lhs)) => {
                graph.units[rhs_id] = rhs_coef;
                for &(id, coef) in lhs {
                    graph.adj_list[rhs_id].push((id, coef));
                }
            }
            _ => unreachable!(),
        }
    }

    let sorted = topological_sort(&graph);

    println!("PART1 {}", required_ore(1, &sorted, &graph));

    let mut hi = 1;

    while required_ore(hi, &sorted, &graph) < 1000000000000 {
        hi *= 2;
    }

    let mut lo = hi / 2;

    while hi - 1 > lo {
        let mid = (hi - lo) / 2 + lo;
        let ore_count = required_ore(mid, &sorted, &graph);
        if ore_count > 1000000000000 {
            hi = mid;
        } else if ore_count == 1000000000000 {
            lo = mid;
            break;
        } else {
            lo = mid;
        }
    }
    println!("PART2 {}", lo);
    
}
