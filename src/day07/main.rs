use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Pointer};
use std::fs;
use std::ops::{Index, Sub};

#[derive(Debug, PartialEq, PartialOrd, Hash, Ord, Eq, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    fn bellow(&self) -> Position {
        let Self(x, y) = self;

        Position(*x, *y+1)
    }
    fn sides(&self) -> Vec<Position> {
        let Self(x, y) = self;
        
        vec![
            Position(*x-1, *y),
            Position(*x+1, *y),
        ]
    }
}

#[derive(Debug)]
struct State {
    splits: HashSet<Position>,
    start: Position,
    
    used_splits: HashSet<Position>,
    seen: HashSet<Position>,
    
    max: Position,
    paths: usize
}

impl State {
    fn in_space(&self, p: Position) -> bool {
        let Position(x, y) = p;
        let Position(max_x, max_y) = self.max;
        
        x >= 0 && x <= max_x && y >= 0 && y <= max_y
        
    }
    
    fn run(&mut self) {
        let mut discover = vec![self.start];
        
        while let Some(p) = discover.pop() {
            if !self.in_space(p) {
                self.paths += 1;
                continue
            }
            if self.seen.contains(&p) {
                continue
            }
            self.seen.insert(p);
            
            let is_split = self.splits.contains(&p);
            if is_split {
                self.used_splits.insert(p);
                discover.extend(p.sides());
            } else {
                discover.push(p.bellow());
            }
            // println!("{}", self);
        }
    }
    
    fn part2(&mut self) -> u64 {
        
        let mut s: Vec<_> = self.splits.iter().collect();
        s.sort_by_key(|Position(x, y)| (y, x));
        
        let mut counts = HashMap::<i32, u64>::new();
        counts.insert(s.first().unwrap().0, 1);
        
        for split in s {
            let &Position(x, _) = split;
            // println!("checking split on {x}");
            let curr = counts[&x];
            // println!("dividing at {x} value={curr}");
            for Position(neigh_x, _) in split.sides() {
                // println!("{:?}", neigh);
                // println!("adding {curr} to {neigh_x}");
                counts.entry(neigh_x).and_modify(|v| *v += curr).or_insert(curr);
            }
            counts.entry(x).and_modify(|v| *v = 0);
            // dbg!(&counts);
        }
        
        counts.values().sum()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Position(max_x, max_y) = self.max;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Position(x, y);
                let ch = if self.start == pos {
                    'S'
                } else if self.used_splits.contains(&pos) {
                    '^'
                } else if self.splits.contains(&pos) {
                    '^'
                } else if self.seen.contains(&pos) {
                    '|'
                } else {
                    '.'
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



fn parse(input: &str) -> State {
    let active_pos = &RefCell::<Option::<Position>>::new(None);
    
    let splits: HashSet<_> = input.lines().into_iter().enumerate().filter_map(move |(y, line)| {
        // if !line.contains('^') &&  !line.contains('S') {
        //     return None;
        // }
        Some(line.chars().enumerate().filter_map(move |(x, c)| {
            if c == 'S' {
                *active_pos.borrow_mut() = Some(Position(x as i32, y as i32));
            }
            (c == '^').then_some(Position(x as i32, y as i32))
        }))
    }).flatten().collect();
    
    let lines = input.lines().collect::<Vec<_>>();
    let max = Position(
        lines.first().unwrap().len() as i32 - 1,
        lines.len() as i32 - 1,
    );
        
    State{
        start: active_pos.borrow().unwrap(),
        splits,
        max,

        used_splits: Default::default(),
        seen: Default::default(),
        paths: 0,
    }

}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day07/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    let mut state = parse(&input);

    state.run();
    println!("splits: {}", state.used_splits.len());
    println!("paths: {}", state.part2());
        
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        
        let mut state = parse(input);
        // state.run();
        // assert_eq!(state.used_splits.len(), 21);
        
        assert_eq!(state.part2(), 40);
    }
    #[test]
    fn test_simple() {
        let input = "\
...S...
...^...
..^.^..
.......
";
        
        let mut state = parse(input);
        state.run();
        assert_eq!(state.used_splits.len(), 3);
        assert_eq!(state.paths, 4);
    }

}