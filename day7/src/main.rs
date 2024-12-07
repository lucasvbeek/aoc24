use itertools::Itertools;
const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let equations = parse(input);
    let operations = vec![Operation::Add, Operation::Multiply];

    equations
        .iter()
        .filter(|e| is_valid_equation(e, &operations))
        .map(|e| e.0)
        .sum()
}

fn process_p2(input: &str) -> usize {
    let equations = parse(input);
    let operations = vec![Operation::Add, Operation::Multiply, Operation::Concat];

    equations
        .iter()
        .filter(|e| is_valid_equation(e, &operations))
        .map(|e| e.0)
        .sum()
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .split("\n")
        .map(|equation| {
            let (value, parts) = equation.split_once(": ").unwrap();
            (
                value.parse::<usize>().unwrap(),
                parts
                    .split(" ")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
    Concat,
}

fn is_valid_equation(equation: &(usize, Vec<usize>), operations: &Vec<Operation>) -> bool {
    let (value, parts) = equation;
    let combinations: Vec<_> = (0..parts.len() - 1)
        .map(|_| operations)
        .multi_cartesian_product()
        .collect();

    combinations.iter().any(|c| {
        let mut sum = parts[0];
        for (i, part) in parts.iter().skip(1).enumerate() {
            match c[i] {
                Operation::Multiply => sum *= part,
                Operation::Add => sum += part,
                Operation::Concat => {
                    sum = (sum.to_string() + &part.to_string())
                        .parse::<usize>()
                        .unwrap()
                }
            }
        }
        &sum == value
    })
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 3749);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 11387);
    }
}
