use std::fs;


type Position = u16;
#[derive(Debug, PartialEq, Eq)]
enum Cmd {
    Left(u16),
    Right(u16),
}


const START: Position = 50;
const DOMAIN: Position = 100;

impl Cmd {
    fn from_str(s: &str) -> Option<Cmd> {
        let (dir, dist) = s.split_at(1);
        let dist: u16 = dist.parse().ok()?;
        match dir {
            "L" => Some(Cmd::Left(dist)),
            "R" => Some(Cmd::Right(dist)),
            _ => None,
        }
    }

    fn apply(&self, state: &Position) -> Position {
        match self {
            Cmd::Left(step) => (state+DOMAIN - (step % DOMAIN)) % DOMAIN,
            Cmd::Right(step) => (state.wrapping_add(DOMAIN).wrapping_add(step % DOMAIN)) % DOMAIN,
        }
    }

    fn apply_zero_counting(&self, state: &Position) -> (Position, u16) {
        match &self {
            Cmd::Left(step) => {
                let diff = (*state as i16) - (*step as i16);

                // bleh
                (self.apply(state), if *state != 0 && diff <= 0 {1} else {0} + (diff.abs() as u16) / DOMAIN)
            }
            Cmd::Right(step) => (self.apply(state), (state + step) / DOMAIN),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day01/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    println!(
        "{}",
        solve(&input).filter_map(|p| if p == 0 {Some(p)} else {None}).count()
    );
    println!(
        "{}",
        solve2(&input).map(|(_, c)| c).sum::<u16>(),
    );

    Ok(())
}

fn solve(input: &str) -> impl Iterator<Item=Position> {
    input
        .lines()
        .filter_map(|l| Cmd::from_str(l))
        .scan(START, |state, cmd| {
            *state = cmd.apply(state);
            Some(*state)
        })
        .into_iter()
}

fn solve2(input: &str) -> impl Iterator<Item=(Position, u16)> {
    input
        .lines()
        .filter_map(|l| Cmd::from_str(l))
        .scan((START, 0), |(pos, _), cmd| {
            let (new_pos, new_zeroes) = cmd.apply_zero_counting(pos);
            *pos = new_pos;
            Some((
                new_pos,
                new_zeroes,
            ))
        })
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_from_str() {
        assert_eq!(Cmd::from_str("L10"), Some(Cmd::Left(10)));
        assert_eq!(Cmd::from_str("R20"), Some(Cmd::Right(20)));
        assert_eq!(Cmd::from_str("X30"), None);
    }

    #[test]
    fn test_cmd_apply() {
        assert_eq!(Cmd::Left(10).apply(&50), 40);
        assert_eq!(Cmd::Right(20).apply(&50), 70);
        assert_eq!(Cmd::Left(60).apply(&50), 90);
        assert_eq!(Cmd::Right(60).apply(&50), 10);


        assert_eq!(Cmd::Left(68).apply(&50), 82);
        assert_eq!(Cmd::Left(30).apply(&82), 52);
        assert_eq!(Cmd::Right(52).apply(&48), 0);
    }


    #[test]
    fn test_cmd_apply_zero_counting() {
        assert_eq!(Cmd::Left(68).apply_zero_counting(&50), (82, 1));
        assert_eq!(Cmd::Right(60).apply_zero_counting(&95), (55, 1));
        assert_eq!(Cmd::Left(82).apply_zero_counting(&14), (32, 1));

        assert_eq!(Cmd::Right(1000).apply_zero_counting(&50), (50, 10));

        // assert_eq!(Cmd::Right(52).apply(48), 0);

        // 14 -  L82 -> 1
        // 14 - L182 -> 2
    }

    #[test]
    fn text_example() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let result: Vec<Position> = solve(input).collect();
        insta::assert_snapshot!(format!("{:?}", result), @"[82, 52, 0, 95, 55, 0, 99, 0, 14, 32]");
    }
    #[test]
    fn text_example2() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let count: u16 = solve2(input).map(|(_, c)| c).sum();
        insta::assert_snapshot!(count, @"6");
    }
    #[test]
    fn text_example3() {
        let input = "\
L50
R50
";
        let count: u16 = solve2(input).map(|(_, c)| c).sum();
        insta::assert_snapshot!(count, @"1");
    }
    #[test]
    fn text_example4() {
        let input = "\
L150
L50
";
        let count: u16 = solve2(input).map(|(_, c)| c).sum();
        insta::assert_snapshot!(count, @"2");
    }
    #[test]
    fn text_example5() {
        let input = "\
L150
R50
";
        let count: u16 = solve2(input).map(|(_, c)| c).sum();
        insta::assert_snapshot!(count, @"2");
    }
    #[test]
    fn text_example6() {
        let input = "\
R150
L50
";
        let count: u16 = solve2(input).map(|(_, c)| c).sum();
        insta::assert_snapshot!(count, @"2");
    }
}
