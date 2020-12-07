use std::collections::{HashMap, HashSet};

use crate::lib;

pub const INPUT_FILE: &str = "input/7.txt";


// plan: parse each line and extract a graph vertex out of it.
// x can contain y => y -> x
// then walk the graph starting from shiny gold, and keep track of which nodes are visited.
// Then return the number of visited nodes.

// plan for second problem: now we have to invert the graph (or rather, NOT invert the source graph)
// add a flag for that
// then, we must also keep track of the vertices' values, maybe we have to make the walk function recursive

// drab plum bags contain 5 clear turquoise bags, 5 striped aqua bags, 4 dotted gold bags, 4 plaid chartreuse bags.
// dark aqua bags contain 3 striped white bags.
// posh black bags contain no other bags.
fn process_line(mut vertices: HashMap<String, Vec<(usize, String)>>, line: String, first: bool) -> HashMap<String, Vec<(usize, String)>> {

    // Get rid of empty bags
    if line.contains("no other bags") {
        return vertices;
    }

    let simple = line
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "");

    let mut words = simple
        .split(" contain ")
        .map(|s| s.to_string());

    let container = words.next().unwrap();
    let contents = words.next().unwrap();

    if first {

        for bags in contents.split(", ") {
            let mut number_containee = bags.splitn(2, " ");
            let number = number_containee.next().unwrap().parse::<usize>().unwrap();
            let containee = number_containee.next().unwrap();
            let mut neighbors = vertices.remove(containee).unwrap_or(vec![]);
            neighbors.push((number, container.to_string()));
            vertices.insert(containee.to_string(), neighbors);
        }

    } else {

        let mut neighbors: Vec<(usize, String)> = vec![];

        for bags in contents.split(", ") {
            let mut number_containee = bags.splitn(2, " ");
            let number = number_containee.next().unwrap().parse::<usize>().unwrap();
            let containee = number_containee.next().unwrap();
            neighbors.push((number, containee.to_string()));
        }

        vertices.insert(container.to_string(), neighbors);


    }

    vertices

}

fn populate(lines: impl Iterator<Item=String>, first: bool) -> HashMap<String, Vec<(usize, String)>> {
    lines.fold(HashMap::new(), |acc, cur| process_line(acc, cur, first))
}

/// Count the amount of vertices that can be reached starting at "shiny gold"
fn walk1(vertices: HashMap<String, Vec<(usize, String)>>) -> usize {

    let mut visited: HashSet<String> = HashSet::new();

    let mut stack: Vec<String> = vec!["shiny gold".to_string()];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }
        if let Some(neighbors) = vertices.get(&current) {
            stack.extend(neighbors.into_iter().map(|(_, name)| name).cloned().collect::<Vec<String>>());
        }
        visited.insert(current);
    }

    visited.len()

}

fn walk2(vertices: &HashMap<String, Vec<(usize, String)>>, vertex: (usize, String), accumulative: usize) -> usize {

    let (vertex_number, vertex_name) = vertex;

    let current = vertex_number * accumulative;

    current + if let Some(neighbors) = vertices.get(&vertex_name) {
        neighbors.into_iter().cloned().map(|n| walk2(vertices, n.clone(), current)).sum()
    } else {
        0
    }

}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(walk1(populate(lib::read_lines(file)?, true)) - 1)
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(walk2(&populate(lib::read_lines(file)?, false), (1, "shiny gold".to_string()), 1) - 1)
}

#[cfg(test)]
mod test {
    use super::{walk1, walk2, populate};

    const INPUT: &str =
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn run1() {
        assert_eq!(4, walk1(populate(INPUT.lines().map(|s| s.to_string()), true)) - 1);
    }

    #[test]
    fn run2() {
        assert_eq!(32, walk2(&populate(INPUT.lines().map(|s| s.to_string()), false), (1, "shiny gold".to_string()), 1) - 1);
    }

    const INPUT_2: &str =
"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn run2_2() {
        assert_eq!(126, walk2(&populate(INPUT_2.lines().map(|s| s.to_string()), false), (1, "shiny gold".to_string()), 1) - 1);
    }

}
