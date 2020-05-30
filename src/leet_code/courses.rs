use std::collections::HashSet;

// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge/538/week-5-may-29th-may-31st/3344/
// Does a topological sort to detect cycles
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // Set-up the stuff for topological sort
    let mut all_edges = prerequisites
        .iter()
        .map(|prereq| prereq)
        .collect::<HashSet<&Vec<i32>>>();
    let mut incoming = all_edges
        .clone()
        .iter()
        .fold(HashSet::new(), |mut acc, prereq| {
            acc.insert(prereq[1]);
            acc
        });
    let mut no_incoming = (0..num_courses)
        .filter(|x| !incoming.contains(x))
        .collect::<Vec<i32>>();
    let mut visited: HashSet<i32> = HashSet::new();

    // Topological sort (Kahn's algorithm)
    // I'm not really sorting since I'm just putting them into a HashSet, but I don't need to
    // I just am using the algo to check for if the courses are a DAG
    while let Some(node) = no_incoming.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        all_edges = all_edges
            .iter()
            .filter(|prereq| prereq[0] != node)
            .map(|prereq| *prereq)
            .collect::<HashSet<&Vec<i32>>>();

        incoming = all_edges
            .clone()
            .iter()
            .fold(HashSet::new(), |mut acc, prereq| {
                acc.insert(prereq[1]);
                acc
            });
        no_incoming = (0..num_courses)
            .filter(|x| !incoming.contains(x))
            .collect::<Vec<i32>>();
    }

    all_edges.is_empty()
}
