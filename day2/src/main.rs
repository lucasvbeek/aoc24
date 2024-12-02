const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = parse_reports(input);

    let safe_reports: Vec<&Vec<i32>> = reports.iter().filter(|r| exactly_safe(r)).collect();
    safe_reports.len()
}

fn process_p2(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = parse_reports(input);

    let safe_reports: Vec<&Vec<i32>> = reports.iter().filter(|r| somewhat_safe(r)).collect();
    safe_reports.len()
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input.split("\n").map(|r| r.split(" ").map(|l| l.parse::<i32>().unwrap()).collect()).collect()
}

fn exactly_safe(report: &Vec<i32>) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|w| w[0] - w[1]).collect();

    let all_positive_or_negative = differences.iter().all(|d| d.is_positive()) || differences.iter().all(|d| d.is_negative());
    let all_diff_size_ok = differences.iter().all(|d| d.abs() >= 1 && d.abs() <= 3);

    all_positive_or_negative && all_diff_size_ok
}

fn somewhat_safe(report: &Vec<i32>) -> bool {
    if exactly_safe(report) {
        return true;
    }

    for (i, _) in report.iter().enumerate() {
        let mut r = report.clone();
        r.remove(i);
        if exactly_safe(&r) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 2);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 4);
    }
}