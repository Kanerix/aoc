use std::cmp::Ordering;

type Report = Vec<i32>;

pub fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|l| l.split(' ').filter_map(|e| e.parse().ok()).collect())
        .collect()
}

pub fn part1(input: Vec<Report>) -> usize {
    input
        .into_iter()
        .filter(|r| {
            let mut e = r.iter();
            let mut l = match e.next() {
                Some(v) => v,
                None => return false,
            };

            let mut ord: Option<Ordering> = None;

            for c in e {
                if let Some(ord) = ord {
                    if l.cmp(c) != ord {
                        return false;
                    }
                } else {
                    ord = Some(l.cmp(c));
                }

                if !(1..4).contains(&(l - c).abs()) {
                    return false;
                }

                l = c;
            }

            true
        })
        .count()
}

pub fn part2(input: Vec<Report>) -> usize {
    input
        .into_iter()
        .filter(|r| {
            let mut e = r.iter();
            let mut l = match e.next() {
                Some(v) => v,
                None => return false,
            };

            let mut ord: Option<Ordering> = None;
            let mut skip = true;

            for c in e {
                if let Some(o) = ord {
                    if l.cmp(c) != o {
                        if skip { skip = false; ord = None; continue; }
                        return false
                    }
                } else {
                    ord = Some(l.cmp(c));
                }

                if !(1..4).contains(&(l - c).abs()) {
                    if skip { skip = false; continue; }
                    return false
                }

                l = c;
            }

            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day2/input1.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 2)
    }

    #[test]
    fn part1_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day2/input2.txt"
        ));
        let input = parse(raw);
        let result = part1(input);
        assert_eq!(result, 306)
    }

    #[test]
    fn part2_input1() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day2/input1.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 4)
    }

    #[test]
    fn part2_input2() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/day2/input2.txt"
        ));
        let input = parse(raw);
        let result = part2(input);
        assert_eq!(result, 353)
    }
}
