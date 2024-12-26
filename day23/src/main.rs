use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    //let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let t = Instant::now();
    println!("{}", part1(input));
    part2(input);
    dbg!(t.elapsed());
}

fn part2(input: &str) {
    let mut graph = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph
                .entry(a.to_string())
                .or_insert(HashSet::new())
                .insert(b.to_string());
            graph
                .entry(b.to_string())
                .or_insert(HashSet::new())
                .insert(a.to_string());
        });
    let mut cliques = Vec::new();
    bron_kerbosch(
        &mut HashSet::new(),
        graph.keys().cloned().collect(),
        HashSet::new(),
        &graph,
        &mut cliques,
    );
    let mut x = cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    x.sort();
    for computer in x {
        print!("{computer},");
    }
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    while let Some(v) = p.iter().next().cloned() {
        p.remove(&v);
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        let new_p = p
            .intersection(graph.get(&v).unwrap_or(&HashSet::new()))
            .cloned()
            .collect();
        let new_x = x
            .intersection(graph.get(&v).unwrap_or(&HashSet::new()))
            .cloned()
            .collect();
        bron_kerbosch(&mut new_r, new_p, new_x, graph, cliques);
        x.insert(v);
    }
}

fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(a, b)| {
            map.entry(a).or_insert(HashSet::new()).insert(b);
            map.entry(b).or_insert(HashSet::new()).insert(a);
        });
    let mut set = HashSet::new();
    map.keys().for_each(|&c1| {
        for &c2 in map.get(c1).unwrap().iter().filter(|&&c| c != c1) {
            for &c3 in map.get(c2).unwrap().iter().filter(|&&c| c != c1 && c != c2) {
                if map.get(c3).unwrap().contains(c1) {
                    assert!(c1 != c3 && c1 != c2 && c2 != c3);
                    let mut x = vec![c1.to_string(), c2.to_string(), c3.to_string()];
                    x.sort();
                    set.insert(x);
                }
            }
        }
    });

    set.into_iter()
        .filter(|vec| vec.iter().any(|computer| computer.starts_with("t")))
        .count()
}
