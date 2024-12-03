use regex::Regex;

const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    count(input)
}

fn process_p2(input: &str) -> usize {
    let mut matches = [input.match_indices("do()").collect::<Vec<(usize, &str)>>(), input.match_indices("don't()").collect()].concat();
    matches.sort_by_key(|m| m.0);

    matches.iter().enumerate().map(|(i, (pos, str))| {
        if *str == "don't()" { return 0; }

        if i == matches.len() - 1 {
            return count(&input[pos.clone()..]);
        }

        count(&input[pos.clone()..matches[i+1].0])
    }).sum::<usize>() + count(&input[0..matches[0].0])
}

fn count(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input).map(|capture| {
        let (_, [first, second]) = capture.extract();
        first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap()
    }).sum()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 161);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 48);
    }
}