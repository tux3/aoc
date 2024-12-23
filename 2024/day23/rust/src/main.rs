#![feature(let_chains)]

use anyhow::Result;
use itertools::Itertools;
use multimap::MultiMap;
use std::collections::HashSet;

fn solve_p1(edges: &MultiMap<String, String>, vertices: &Vec<String>) -> usize {
    let mut edges_dup = edges.clone();
    let mut triangles = Vec::new();
    for v in vertices {
        let Some(v_neigh) = edges_dup.get_vec(v) else {
            continue;
        };
        let mut mark = v_neigh.into_iter().cloned().collect::<HashSet<_>>();
        for u in v_neigh {
            for w in edges_dup.get_vec(u).unwrap() {
                if mark.contains(w) {
                    triangles.push((v.clone(), u.clone(), w.clone()));
                }
            }
            mark.remove(u);
        }
        edges_dup.remove(v);
        edges_dup.retain(|a, b| a != v && b != v);
    }

    let tris_t = triangles
        .iter()
        .filter(|(u, v, w)| u.starts_with('t') || v.starts_with('t') || w.starts_with('t'))
        .collect::<Vec<_>>();

    tris_t.len()
}

// Welcome to clone city, population: String
fn bron_kerbosch(
    edges: &MultiMap<String, String>,
    vertices: &Vec<String>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
) -> Option<HashSet<String>> {
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }
    let mut max_clique: Option<HashSet<String>> = None;
    while !p.is_empty() {
        let v = p.iter().next().unwrap();
        let mut r2 = r.clone();
        r2.insert(v.to_owned());
        let neigh_v = edges
            .get_vec(v)
            .map(|n| HashSet::from_iter(n.iter().cloned()))
            .unwrap_or_default();
        let p2 = p.intersection(&neigh_v).cloned().collect::<HashSet<_>>();
        let x2 = x.intersection(&neigh_v).cloned().collect::<HashSet<_>>();
        if let Some(clique) = bron_kerbosch(edges, vertices, r2, p2, x2) {
            if let Some(best) = &max_clique
                && clique.len() > best.len()
            {
                max_clique = Some(clique)
            } else if max_clique.is_none() {
                max_clique = Some(clique)
            }
        }
        let v = v.to_owned();
        p.remove(&v);
        x.insert(v);
    }
    max_clique
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let pairs = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once('-').unwrap();
            (l.to_string(), r.to_string())
        })
        .collect::<Vec<_>>();

    let edges = pairs
        .into_iter()
        .flat_map(|(l, r)| [(l.clone(), r.clone()), (r, l)])
        .collect::<MultiMap<String, String>>();

    let mut vertices = edges.keys().cloned().collect::<Vec<_>>();
    vertices.sort_by_key(|v| edges.get_vec(v).unwrap().len());

    println!("Tris starting with t: {}", solve_p1(&edges, &vertices));

    let r = HashSet::new();
    let p = HashSet::from_iter(vertices.iter().cloned());
    let x = HashSet::new();
    let max_clique = bron_kerbosch(&edges, &vertices, r, p, x).unwrap();
    let pass = max_clique.into_iter().sorted().join(",");
    println!("LAN password: {pass}");
    Ok(())
}
