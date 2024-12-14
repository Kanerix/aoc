pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(mut input: Vec<&str>) -> usize {
    let mut rev = input.clone();
    rev.reverse();
    input.append(&mut rev);

    0
}

pub fn part2(mut input: Vec<&str>) -> usize {
    let mut rev = input.clone();
    rev.reverse();
    input.append(&mut rev);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day4/input1.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 18)
    }

    #[test]
    fn part1_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day4/input2.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 306)
    }

    #[test]
    fn part2_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day4/input1.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 4)
    }

    #[test]
    fn part2_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day4/input2.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 381)
    }
}
