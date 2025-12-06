
use std::fs;
use itertools::Itertools;
use rayon::prelude::*;


fn solve(input: &str) -> u64 {
    return solve2(input, 2);
}
fn solve2(input: &str, k: usize) -> u64 {

    let numbers = input
        .as_bytes()
        .iter()
        .map(|c| c - ('0' as u8))
        .collect::<Vec<_>>();

    let mut res = vec![];
    let l = numbers.len();
    let mut left_bound = 0;
    while res.len() != k {
        // l=10, k=5, |res|=2
        let avail_count = l - (k - res.len() - 1);
        let avail = &numbers[left_bound..avail_count];
        let max = avail.iter().max().unwrap();
        let i = avail.iter().position(|v| v == max).unwrap();
        res.push(numbers[left_bound+i]);
        left_bound += i + 1;
    }
    as_number(res.as_slice())
}

//
fn as_number(nums: &[u8]) -> u64 {
    nums.iter().fold(0u64, |acc, &e| 10*acc + (e as u64))
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day03/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    // println!("{}", input.lines().map(solve).sum::<u32>());
    println!(
        "{}",
        input
            .lines()
            .map(|l| solve2(l, 2))
            .sum::<u64>(),
    );
    println!(
        "{}",
        input
            .lines()
            .map(|l| solve2(l, 12))
            .sum::<u64>(),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        assert_eq!(solve("987654321111111"), 98);
        assert_eq!(solve("811111111111119"), 89);
        assert_eq!(solve("234234234234278"), 78);
        assert_eq!(solve("818181911112111"), 92);
    }
    #[test]
    fn test_demo2() {
        assert_eq!(solve2("987654321111111", 12), 987654321111);
        assert_eq!(solve2("811111111111119", 12), 811111111119);
        assert_eq!(solve2("234234234234278", 12), 434234234278);
        assert_eq!(solve2("818181911112111", 12), 888911112111);
    }
}
