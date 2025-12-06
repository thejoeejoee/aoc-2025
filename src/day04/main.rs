use std::collections::HashSet;
use std::fs;

fn neigh((x, y): &(i16, i16)) -> [(i16, i16); 8] {
    [
        (x-1, y-1),
        (x-1, y+0),
        (x-1, y+1),
        (x+0, y-1),
        (x+0, y+1),
        (x+1, y-1),
        (x+1, y+0),
        (x+1, y+1),
    ]
}


fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day04/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    let m = parse(&input);
    println!("{}", m.available().len());
    println!("{}", m.exhaustive().len());


    Ok(())
}

struct Space {
    blocks: HashSet<(i16, i16)>
}

impl Space {
    fn available(&self) -> Vec<(i16, i16)> {
        self.blocks.iter().filter_map(|&p| {
            (
                neigh(&p).iter().filter(|np| {self.blocks.contains(np)}).count()
                <
                4
            ).then_some(p)
        }).collect::<Vec<_>>()
    }


    fn exhaustive(mut self) -> HashSet<(i16, i16)> {
        let mut total = HashSet::<(i16, i16)>::default();
        let mut rem = self.available();
        total.extend(rem.clone());
        while !rem.is_empty() {
            // eprintln!("{}", rem.len());
            self.blocks = self.blocks
                .into_iter()
                .filter(|b| !rem.contains(b))
                .collect();
            rem = self.available();
            total.extend(rem.clone());
        }

        total
    }


}

fn parse(input: &str) -> Space {
    let m: HashSet<_> = input.lines().into_iter().enumerate().map(|(y, line)| {
        line.bytes().enumerate().filter_map(move |(x, c)| {
            (c == b'@').then_some((x as i16, y as i16))
        })
    }).flatten().collect();

    Space{ blocks: m }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let demo = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let m = parse(demo);

        assert_eq!(m.blocks.len(), 71);
        assert_eq!(m.available().len(), 13);
        assert_eq!(m.exhaustive().len(), 43);
    }

}
