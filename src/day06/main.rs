use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::ops::{Index, Range, Sub};
use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::*;



fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day06/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    let parsed = parse(&input);

    println!("{}", parsed.iter().map(|b| b.compute()).sum::<u64>());
    println!("{}", parsed.iter().map(|b| b.compute_col()).sum::<u64>());

    Ok(())
}

#[derive(Debug)]
struct Block {
    lines: Vec<String>,
    op: char,
}

impl Block {
    fn compute(&self) -> u64 {
        let add = self.op == '+';
        self.lines.iter()
            .map(|l| l.trim().parse::<u64>().unwrap())
            .fold(
                if add {0} else {1},
                |acc, v| if add {acc+v} else {acc*v}
            )
    }
    
    fn compute_col(&self) -> u64 {
        let add = self.op == '+';
        
        (0..self.lines.last().unwrap().len())
            .map(
                |col_i|
                    self.lines.iter()
                        .map(|l| l.chars().nth(col_i).unwrap())
                        .collect::<Vec<_>>()
            )
            .map(|n| 
                n.iter().join("").trim().parse::<u64>().unwrap()
            )
            .fold(
                if add {0} else {1},
                |acc, v| if add {acc+v} else {acc*v}
            )
    }
}

fn parse(input: &str) -> Vec<Block> {
    let lines: Vec<_> = input.lines().collect();
    let blocks = &lines[..lines.len()-1];
    lines
        .last().unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .chain(vec![
            (1+lines.iter().map(|l| l.len()).max().unwrap(),
             '_',
        )])
        .tuple_windows::<(_, _)>()
        .map(|((my_i, my_op), (next_i, _))| {
            Block {
                lines: blocks.iter().map(|l| {
                    String::from(&l[my_i..next_i-1])
                }).collect::<Vec<_>>(),
                op: my_op,
            }
        }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let input = include_str!("demo");
        let parsed = parse(input);

        // dbg!(parsed);
        
        assert_eq!(parsed.iter().map(|b| b.compute()).sum::<u64>(), 4277556);
    }
    #[test]
    fn test_demo2() {
        let input = include_str!("demo");
        let parsed = parse(input);

        // dbg!(parsed);
        
        assert_eq!(parsed.iter().map(|b| b.compute_col()).sum::<u64>(), 3263827);
    }

}