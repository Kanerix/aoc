use std::sync::OnceLock;

use regex::Regex;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: Vec<&str>) -> u32 {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let re = REGEX.get_or_init(|| {
        Regex::new(r"mul\((:?\d{1,3}),(:?\d{1,3})\)").unwrap()
    });

    input.iter().map(|l| {
        re.captures_iter(l).filter_map(|c| {
                let a = c[1].parse::<u32>().ok()?;
                let b = c[2].parse::<u32>().ok()?;
                Some(a*b)
            })
            .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part2(input: Vec<&str>) -> u32 {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let re = REGEX.get_or_init(|| {
        Regex::new(r"mul\((:?\d{1,3}),(:?\d{1,3})\)").unwrap()
    });

    input.iter().map(|l| {
        let mut mul = false;
        re.captures_iter(l).filter_map(|c| {
                if mul {
                    return None
                }

                let a = c[1].parse::<u32>().ok()?;
                let b = c[2].parse::<u32>().ok()?;
                return Some(a*b)
            })
            .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day3/input1.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 48)
    }

    #[test]
    fn part1_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day3/input2.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 306)
    }

    #[test]
    fn part2_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day3/input1.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 4)
    }

    #[test]
    fn part2_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day3/input2.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 381)
    }
}
