use petgraph::{Graph, Undirected};

#[aoc_generator(day12)]
pub fn parse_graph_edges(input: &str) -> (Vec<String>, Graph<(), (), Undirected, usize>) {
    let mut names = Vec::new();
    let graph = Graph::from_edges(
        input
            .lines()
            .filter_map(|l| l.split_once('-'))
            .map(|(x, y)| (find_or_insert(&mut names, x), find_or_insert(&mut names, y))),
    );
    (names, graph)
}

#[aoc(day12, part1)]
pub fn part1(input: &(Vec<String>, Graph<(), (), Undirected, usize>)) -> usize {
    // How many parts from the node named start to the node named end, noting
    // that a path may not visit a node with a lower case name more than once?
    let start = find(&input.0, "start").unwrap();
    depth_first_search_end(&input.1, &input.0, &mut Vec::new(), start)
}

#[aoc(day12, part2)]
pub fn part2(input: &(Vec<String>, Graph<(), (), Undirected, usize>)) -> usize {
    // How many parts from the node named start to the node named end, noting
    // that a path may not visit a node with a lower case name more than once,
    // except for one lower case node that may be visited twice?
    let start = find(&input.0, "start").unwrap();
    depth_first_search_end_exception(&input.1, &input.0, &mut Vec::new(), start)
}

fn find_or_insert(v: &mut Vec<String>, element: &str) -> usize {
    v.iter()
        .enumerate()
        .find(|(_, elt)| element == **elt)
        .map(|elt| elt.0)
        .unwrap_or_else(|| {
            v.push(element.into());
            v.len() - 1
        })
}

fn find(v: &[String], element: &str) -> Option<usize> {
    v.iter()
        .enumerate()
        .find(|(_, elt)| element == **elt)
        .map(|elt| elt.0)
}

/// Assumes we can just check the first character, and that the string isn't empty
fn is_uppercase(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_uppercase()
}

/// How many parts from the node named `from` to the node named end, noting
/// that a path may not visit a node with a lower case name more than once?
fn depth_first_search_end(
    graph: &Graph<(), (), Undirected, usize>,
    names: &[String],
    visited: &mut Vec<usize>,
    source: usize,
) -> usize {
    let mut n_paths = 0;
    visited.push(source);
    for i in graph.neighbors(source.into()) {
        if names[i.index()] == "end" {
            n_paths += 1;
        } else if is_uppercase(&names[i.index()]) || !visited.contains(&i.index()) {
            n_paths += depth_first_search_end(graph, names, visited, i.index());
        }
    }
    assert_eq!(visited.pop(), Some(source));
    n_paths
}

/// How many parts from the node named `from` to the node named end, noting
/// that a path may not visit a node with a lower case name more than once,
/// except for one lower case node that may be visited twice?
fn depth_first_search_end_exception(
    graph: &Graph<(), (), Undirected, usize>,
    names: &[String],
    visited: &mut Vec<usize>,
    source: usize,
) -> usize {
    let mut n_paths = 0;
    visited.push(source);
    for i in graph.neighbors(source.into()) {
        if names[i.index()] == "end" {
            n_paths += 1;
        } else if is_uppercase(&names[i.index()]) || !visited.contains(&i.index()) {
            n_paths += depth_first_search_end_exception(graph, names, visited, i.index());
        } else if names[i.index()] != "start" {
            // It is lowercase, and it was visited, but we haven't used the Special Exception yet!
            // Use the algo from part 1, which will not make more exceptions, to search for "end"
            // from this point onwards
            n_paths += depth_first_search_end(graph, names, visited, i.index());
        }
    }
    assert_eq!(visited.pop(), Some(source));
    n_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_graph_edges(TEST_INPUT)), 10);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_graph_edges(TEST_INPUT)), 36);
    }
}
