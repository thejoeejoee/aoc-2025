#[allow(unused, unused_imports)]
use std::fs;
use std::ops::{Add};
use prime_factorization::Factorization;
use itertools::Itertools;
use num::{range, Integer};

#[derive(Debug, Copy, Clone)]
struct Range(u64, u64);

impl Range {
    fn sum_invalid(&self, only_half_splits: bool) -> u64 {
        let Range(start, end) = self.clone();

        range(start, end + 1).map(|v| -> Option<u64> {
            let (len, splits) = all_splits(v.clone() as u64);
            if splits.is_empty() {
                // single digit
                return None
            }

            splits.into_iter().any(|part_len| {
                if (len / part_len) != 2 && only_half_splits {
                    // only interested in half splits
                    return false
                }

                let uniq_parts = (0..(len / part_len))
                    .map(
                        |s_idx|
                            slice_int(
                                v as u64,
                                ((part_len * s_idx), ((s_idx + 1) * part_len))
                            )
                    )
                    .counts().keys().count();

                return uniq_parts == 1
            }).then_some(v)
        }).flatten().sum()
    }
}

impl TryFrom<&str> for Range {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((l, r)) = value.split_once("-") {
            Ok(Range(
                l.parse::<u64>()?,
                r.parse::<u64>()?,
            ))
        } else {
            Err(anyhow::format_err!("no split available"))
        }

    }
}


fn slice_int(v: u64, (b, t): (u64, u64)) -> u64 {

    // v.mod_floor(&10u64.pow(t)) / 10u64.pow(b)

    let bottom = (v / 10u64.pow(b as u32));
    let res = bottom.mod_floor(&10u64.pow((t - b) as u32));
    // eprintln!("slicing {}[{}:{}] -> {}", v, b, t, res);
    res
}

fn all_splits(input: u64) -> (u64, Vec<u64>) {
    let length = (input as f64).log10().add(1.0).floor() as u64;

    let factors = Factorization::run(length).factors;

    (
        length,
        (1..=factors.len())
            .flat_map(
                |k| factors
                    .iter()
                    .permutations(k)
                    .map(|v| v.iter().fold(1, |acc, &e| acc * e))
            )
            .chain(vec![1])
            .unique()
            .filter(|&v| v != length)
            .sorted()
            .collect()
    )
}

fn to_ranges(input: &str) -> Vec<Range> {
    input
        .replace("\n", "")
        .replace(" ", "")
        .split(",")
        .filter_map(|r| Range::try_from(r).ok())
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day02/input")
        .map_err(|err| anyhow::anyhow!(err))?;

    println!("{}", to_ranges(&input).iter().map(|r| r.sum_invalid(true)).sum::<u64>());
    println!("{}", to_ranges(&input).iter().map(|r| r.sum_invalid(false)).sum::<u64>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    const DEMO_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[test]
    fn text_parse_range() {

        let result = to_ranges(DEMO_INPUT);
        insta::assert_snapshot!(format!("{:?}", result), @"[Range(11, 22), Range(95, 115), Range(998, 1012), Range(1188511880, 1188511890), Range(222220, 222224), Range(1698522, 1698528), Range(446443, 446449), Range(38593856, 38593862), Range(565653, 565659), Range(824824821, 824824827), Range(2121212118, 2121212124)]");
    }

    // https://insta.rs/docs/patterns/
    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    #[rstest]
    #[case(1)]
    #[case(11)]
    #[case(111)]
    #[case(1111)]
    #[case(11111)]
    #[case(111111)]
    #[case(1111111)]
    #[case(11111111)]
    #[case(111111111)]
    #[case(1111111111)]
    #[case(11111111111)]
    #[case(111111111111)]
    #[case(1111111111111)]
    // has to be a better way to do this :-D
    fn test_all_splits(#[case] input: u64) {
        set_snapshot_suffix!("{}", input);
        insta::assert_debug_snapshot!(format!("{:?}", all_splits(input).1));
    }

    #[rstest]
    #[case(1, (0, 0))]
    #[case(12, (1, 0))]
    #[case(1234, (2, 1))]
    #[case(1234, (3, 2))]
    // #[case(123456, (0, 2))]
    fn test_slice_int(#[case] v: u64, #[case] (t, b): (u64, u64)) {
        set_snapshot_suffix!("{}[{}:{}]", v, t, b);
        insta::assert_debug_snapshot!(slice_int(v, (b, t)));
    }

    #[test]
    fn test_edge() {
        assert_eq!(slice_int(123456, (0, 1)), 6);
        assert_eq!(slice_int(123456, (0, 2)), 56);
        assert_eq!(slice_int(123456, (1, 3)), 45);
    }
    #[test]
    fn test_range() {
        assert_eq!(Range(11, 22).sum_invalid(true), 33);
        assert_eq!(Range(95, 115).sum_invalid(true), 99);
        assert_eq!(Range(998, 1012).sum_invalid(true), 1010);
        assert_eq!(Range(1188511880, 1188511890).sum_invalid(true), 1188511885);

        assert_eq!(Range(95, 115).sum_invalid(false), 99+111);
        assert_eq!(Range(565653, 565659).sum_invalid(false), 565656);
        assert_eq!(Range(824824821, 824824827).sum_invalid(false), 824824824);
        assert_eq!(Range(2121212118, 2121212124).sum_invalid(false), 2121212121);

        assert_eq!(Range(11, 22).sum_invalid(false), 33);
        assert_eq!(Range(95, 115).sum_invalid(false), 99+111);
        assert_eq!(Range(998, 1012).sum_invalid(false), 999+1010);
        assert_eq!(Range(1188511880, 1188511890).sum_invalid(false), 1188511885);
        assert_eq!(Range(222220, 222224).sum_invalid(false), 222222);

    }
    #[test]
    fn test_demo() {
        let demo = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        
        assert_eq!(to_ranges(demo).iter().map(|r| r.sum_invalid(true)).sum::<u64>(), 1227775554);
    }
    #[test]
    fn test_demo_2() {
        let demo = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

        assert_eq!(to_ranges(demo).iter().map(|r| r.sum_invalid(false)).sum::<u64>(), 4174379265);
    }
    #[test]
    fn test_length() {
        assert_eq!(all_splits(100).0, 3);
    }
}
