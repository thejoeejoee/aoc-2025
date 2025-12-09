
use std::cmp::{Reverse};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter,};
use std::fs;
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Hash, Ord, Eq, Copy, Clone)]
struct Point(u32, u32, u32);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:},{:},{:}", self.0, self.1, self.2)
    }
}

impl Point {
    fn distance(&self, Point(x, y, z): &Point) -> u64 {
        let Point(mx, my, mz) = self;
        
        (x.abs_diff(*mx) as u64).pow(2) +
        (y.abs_diff(*my) as u64).pow(2) +
        (z.abs_diff(*mz) as u64).pow(2)
    
    }
    
    fn key<'a>(&'a self, other: &'a Point) -> (&'a Point, &'a Point) {
        if self > other {
            (other, self)
        } else {
            (self, other)
        }
    }
}


fn parse(input: &str) -> Vec<Point> {
    input.lines().map(|l| {
        let mut values = l.splitn(3, ",").map(|v| v.parse::<u32>().unwrap());
        
        Point{
            0: values.next().unwrap(),
            1: values.next().unwrap(),
            2: values.next().unwrap(),
        }
    }).collect()
}

fn solve(points: Vec<Point>, mut to_connect: usize) -> usize {
    
    let mut distances: Vec<(&Point, &Point)> = 
        points.iter()
            .combinations(2)
            .map(|p| {
                let (&i, &j) = (p.first().unwrap(), p.last().unwrap());
                (i.key(j), i.distance(j))
            })
            .sorted_by_key(|(_, dist)| Reverse(*dist))
            .map(|(k, _)| k)
            .collect();
    
    let mut circuits = HashMap::<Point, usize>::new();
    
    while to_connect > 0 {
        let (i, j) = distances.pop().unwrap();
        
        if circuits.contains_key(i) && circuits.contains_key(j) {
            // already clustered
            let new_cl = circuits[i];
            let old_cl = circuits[j];
            for &p in circuits.clone().keys() {
                circuits.entry(p).and_modify(|e| {
                    if *e == old_cl {
                        *e = new_cl;
                    }
                })      ;          
            }
            
            println!("already cluster: {}\t{}", i, j);
        } else if circuits.contains_key(i) {
            circuits.insert(*j, circuits[i]);
            println!(" joined cluster: {}\t{}: {}", i, j, circuits[j]);
        } else if circuits.contains_key(j) {
            circuits.insert(*i, circuits[j]);
            println!(" joined cluster: {}\t{}: {}", i, j, circuits[j]);
        } else {
            println!("    new cluster: {}\t{}: {}", i, j, to_connect);
            circuits.insert(*j, to_connect);
            circuits.insert(*i, to_connect);
        }
        to_connect -= 1;
    }

    let top: Vec<_> = circuits
        .values()
        .counts()
        .into_iter().collect::<Vec<_>>() // {circuit: count}
        .into_iter().sorted_by_key(move |(_, count)| Reverse(*count))
        .collect();
    
    let mut top = top.into_iter().map(|(_, c)| c);
    top.next().unwrap() * top.next().unwrap() *top.next().unwrap()

}

fn solve2(points: Vec<Point>) -> usize {
    let mut distances: Vec<((&Point, &Point), u64)> = 
        points.iter()
            .combinations(2)
            .map(|p| {
                let (&i, &j) = (p.first().unwrap(), p.last().unwrap());
                (i.key(j), i.distance(j))
            })
            .sorted_by_key(|(_, dist)| Reverse(*dist))
            .collect();
    
    let mut p2c = HashMap::<&Point, usize>::new();
    let mut cid = 1;
    
    let done = |
        p2c: &HashMap::<&Point, usize>,
        d: &Vec<_>
    | 
        // d.is_empty() && // no remaining
        *p2c.values().counts().values().next().unwrap() == points.len(); // 
    
    // let mut history: Vec<_> = Vec::new();
    
    loop {
        
        let Some(((i, j), distance)) = distances.pop() else {panic!("bad")};
        println!("{} and {} ({distance}): {:?} ->", i, j, p2c.values().counts());
        
        let i_has_cluster = p2c.contains_key(i);
        let j_has_cluster = p2c.contains_key(j);
        
        // if !i_has_cluster && !j_has_cluster {
        //     p2c.insert(i, cid);
        //     p2c.insert(j, cid);
        //     cid += 1;
        //     println!("\tnew cluster")
        // } else {
            let old1 = if i_has_cluster {p2c[i]} else {0};
            let old2 = if j_has_cluster {p2c[j]} else {0};
            
            for (_, c) in p2c.iter_mut() {
                if *c == old1 || *c == old2 {
                    *c = cid;
                }
            }
            p2c.insert(i, cid);
            p2c.insert(j, cid);
            cid += 1;
            if old1 == 0 || old2 == 0 {
                println!("\tINSERT to cluster")    
            } else {
                println!("\tcluster join")
            }
        // }
        
        if done(&p2c, &distances) {
            break
        }
    }
    
    0
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day08/input")
        .map_err(|err| anyhow::anyhow!(err))?;
    
    let m = parse(&input);
    // println!("{}", solve(m.clone(), 1000));
    println!("{}", solve2(m));
       
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let demo = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        
        let points = parse(demo);
        assert_eq!(points.len(), 20);
        
        // assert_eq!(solve(points.clone(), 10), 40);
        assert_eq!(solve2(points), 25272);
    }

}