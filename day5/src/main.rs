const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let (rules, prints) = parse(input);

    prints.iter().filter(|p| is_correct_print(p, &rules)).map(|p| { 
        p.get(p.len() / 2).unwrap()
    }).sum()
}

fn process_p2(input: &str) -> usize {
    let (rules, prints) = parse(input);

    prints.iter().filter(|p| !is_correct_print(p, &rules)).map(|p| {
        let sorted = sort_print(p, &rules);
        *sorted.get(sorted.len() / 2).unwrap()
    }).sum()
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, prints) = input.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules.split("\n").map(|r| r.split_once("|").map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap())).unwrap()).collect();
    let prints: Vec<Vec<usize>> = prints.split("\n").map(|p| p.split(",").map(|s| s.parse().unwrap()).collect::<Vec<usize>>()).collect();
    (rules, prints)
}

fn is_correct_print(print: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    print.is_sorted_by(|&a, &b| rules.iter().filter(|&&r| r == (a, b)).count() > 0)
}

fn sort_print(print: &Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut sorted = print.clone();
    sorted.sort_by(|&a, &b| 0.cmp(&rules.iter().filter(|&&r| r == (a, b)).count()));
    sorted
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 143);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 123);
    }
}