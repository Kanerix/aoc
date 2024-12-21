use std::collections::BTreeMap;

type Locations = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Locations {
    input.lines()
        .filter_map(|line| {
            let mut parts = line
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<u32>>()
                .into_iter();

            match (parts.next(), parts.next()) {
                (Some(l1), Some(l2)) => Some((l1, l2)),
                _ => None,
            }
        })
        .collect()
}

pub fn part1(mut input: Locations) -> u32 {
    input.0.sort();
    input.1.sort();
    Iterator::zip(input.0.iter(), input.1.iter())
        .map(|(a, b)| u32::abs_diff(*a, *b))
        .sum()
}

pub fn part2(input: Locations) -> u32 {
    let mut cache = BTreeMap::<u32, u32>::new();
    input.1.into_iter().for_each(|e| {
        cache.entry(e).and_modify(|c| *c += 1).or_insert(1);
    });
    input.0.into_iter().filter_map(|e| cache.get(&e).map(|v| e * *v)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part1/input1.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 11)
    }

    #[test]
    fn part1_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part1/input2.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 2769675)
    }

    #[test]
    fn part2_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part2/input1.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 31)
    }

    #[test]
    fn part2_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part2/input2.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 24643097)
    }
}
