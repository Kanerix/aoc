use std::cmp::Ordering;

type Report = Vec<i32>;

pub fn parse(input: &str) -> Vec<Report> {
    input.lines()
        .map(|l| l.split(' ').filter_map(|e| e.parse().ok()).collect())
        .collect()
}

pub fn part1(input: Vec<Report>) -> usize {
    input.iter()
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

                if !(0..4).contains(&(l - c).abs()) {
                    return false;
                }

                l = c;
            }

            true
        })
        .count()
}

pub fn part2(input: Vec<Report>) -> usize {
    input.into_iter()
        .filter(|report| {
            let (failed_1, failed_2) = match try_parse_report(report.clone()) {
                Some(v) => v,
                None => return true,
            };

            match try_parse_report(failed_1) {
                Some(_) => (),
                None => return true,
            };

            match try_parse_report(failed_2) {
                Some(_) => (),
                None => return true,
            };

            false
        })
        .count()
}

fn try_parse_report(report: Report) -> Option<(Report, Report)> {
    let mut element = report.iter().enumerate();

    let (_, mut last) = element.next()?;

    let mut ord: Option<Ordering> = None;

    for (i, curr) in element {
        if let Some(ord) = ord {
            if last.cmp(curr) != ord {
                return Some(failed_report(report, i));
            }
        } else {
            ord = Some(last.cmp(curr));
        }

        if !(0..4).contains(&(last - curr).abs()) {
            return Some(failed_report(report, i));
        }

        last = curr;
    }

    None
}

fn failed_report(report: Report, i: usize) -> (Report, Report) {
    let mut report_1 = report.clone();
    report_1.remove(i);
    let mut report_2 = report;
    report_2.remove(i - 1);
    (report_1, report_2)
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
        assert_eq!(result, 381)
    }
}
