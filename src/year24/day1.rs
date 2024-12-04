use std::collections::BTreeMap;

pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| {
            let mut l = line
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<u32>>()
                .into_iter();

            match (l.next(), l.next()) {
                (Some(l1), Some(l2)) => Some((l1, l2)),
                _ => None,
            }
        })
        .collect()
}

pub fn part1(mut input: (Vec<u32>, Vec<u32>)) -> u32 {
    input.0.sort();
    input.1.sort();
    Iterator::zip(input.0.into_iter(), input.1.into_iter())
        .map(|(a, b)| u32::abs_diff(a, b))
        .sum()
}

pub fn part2(input: (Vec<u32>, Vec<u32>)) -> u32 {
    let mut cache = BTreeMap::<u32, u32>::new();
    input.1.into_iter().for_each(|e| {
        cache.entry(e).and_modify(|c| *c += 1).or_insert(1);
    });
    input.0.into_iter().filter_map(|e| match cache.get(&e) {
        Some(v) => Some(e * *v),
        None => None,
    }).sum()
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
        let part1 = part1(input);
        assert_eq!(part1, 11)
    }

    #[test]
    fn part1_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part1/input2.txt"
        ));
        let input = parse(raw);
        let part1 = part1(input);
        assert_eq!(part1, 2769675)
    }

    #[test]
    fn part2_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part2/input1.txt"
        ));
        let input = parse(raw);
        let part1 = part2(input);
        assert_eq!(part1, 31)
    }

    #[test]
    fn part2_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day1/part2/input2.txt"
        ));
        let input = parse(raw);
        let part1 = part2(input);
        assert_eq!(part1, 24643097)
    }
}
