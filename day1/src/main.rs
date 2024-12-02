const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let values: Vec<Vec<&str>> = input.split("\n").map(|l| l.split("   ").collect()).collect();
    
    let mut left: Vec<i32> = values.iter().map(|f| f[0].parse::<i32>().unwrap()).collect();
    left.sort();
    let mut right: Vec<i32> = values.iter().map(|f| f[1].parse::<i32>().unwrap()).collect();
    right.sort();

    let answer: i32 = left.iter().enumerate().map(|(i, val)| (val - right[i]).abs()).sum();

    answer as usize
}

fn process_p2(input: &str) -> usize {
    let values: Vec<Vec<&str>> = input.split("\n").map(|l| l.split("   ").collect()).collect();
    
    let mut left: Vec<i32> = values.iter().map(|f| f[0].parse::<i32>().unwrap()).collect();
    left.sort();
    let mut right: Vec<i32> = values.iter().map(|f| f[1].parse::<i32>().unwrap()).collect();
    right.sort();


    let answer: Vec<usize> = left.iter().map(|val| {
        let find = right.iter().filter(|f| f == &val).count();
        *val as usize * find
    }).collect();
    
    answer.iter().sum()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 11);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 31);
    }
}