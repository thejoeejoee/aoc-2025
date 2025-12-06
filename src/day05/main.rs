use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::ops::{Range, Sub};
use itertools::Itertools;
use rayon::prelude::*;


#[derive(Debug)]
struct State {
    ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

impl State {
    fn fresh(&self) -> usize {
        self.ids.iter().filter(|id| {
            self.ranges.iter().any(|r| r.contains(id))
        }).count()
    }

    fn fresh2(&mut self) -> u64 {
        let mut clear = self.ranges.clone();
        clear.sort_by_key(|r| (r.start.clone(), r.end.clone()));
        clear.dedup_by(|r, l| {
            if l.end >= r.start {
                l.end = max(r.end.clone(), l.end.clone());
                true
            } else {
                false
            }
        });

        clear.iter().map(|r| r.end - r.start).sum()
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day05/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    let mut s = parse(&input);
    println!("{}", s.fresh());
    println!("{}", s.fresh2());

    Ok(())
}

fn parse(input: &str) -> State {
    let mut lines = input.lines().into_iter();

    let ranges: Vec<_> = lines.take_while_ref(|l| !l.is_empty()).collect();
    let ids: Vec<_> = lines.skip(1).collect();

    State{
        ranges: ranges.iter().map(|&r| {
            let Some((s, e)) = r.split_once("-") else { panic!("unparsable {}", r) };
            Range{
                start: s.parse().unwrap(),
                end: e.parse::<u64>().unwrap()+1,
            }
        }).collect(),
        ids: ids.iter().map(|i| i.parse().unwrap()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let demo = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let mut state = parse(demo);

        // assert_eq!(state.fresh(), 3);
        assert_eq!(state.fresh2(), 14);
    }

}
