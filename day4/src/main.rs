use std::vec;

const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let grid = parse_grid(input);

    grid.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, letter)| {
            if letter == &"X" {
                return part_of_xmas(grid.clone(), x, y);
            }
            0
        }).sum::<usize>()
    }).sum()
}

fn process_p2(input: &str) -> usize {
    let grid = parse_grid(input);

    grid.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, letter)| {
            if letter == &"A" && is_mas(grid.clone(), x, y) {
                return 1;
            }
            0
        }).sum::<usize>()
    }).sum()
}

fn parse_grid(input: &str) -> Vec<Vec<&str>> {
    input.split("\n").map(|l| l.split("").collect()).collect()
}

fn part_of_xmas(grid: Vec<Vec<&str>>, x: usize, y: usize) -> usize {
    let options: Vec<[(i32, i32); 3]>  = [
        [(1,0), (2,0), (3,0)], // Regular
        [(-1,0), (-2,0), (-3,0)], // Backwards
        [(0,1), (0,2), (0,3)], // Down
        [(0,-1), (0,-2), (0,-3)], // Up
        [(1,1), (2,2), (3,3)], // Down-right
        [(-1,1), (-2,2), (-3,3)], // Down-left
        [(1,-1), (2,-2), (3,-3)], // Up-right
        [(-1,-1), (-2,-2), (-3,-3)], // Up-left
    ].to_vec();

    options.iter().map(|option| {
        option.iter().enumerate().all(|(i, (x_target, y_target))| {
            let letter = letter_from_grid(grid.clone(), (x as i32 + *x_target) as usize, (y as i32 + *y_target) as usize);
            match i {
               0 => letter == "M",
               1 => letter == "A",
               2 => letter == "S",
               _ => false
            }
        })
    }).filter(|option| *option).count()
}

fn is_mas(grid: Vec<Vec<&str>>, x: usize, y: usize) -> bool {
    let options: Vec<[&str; 4]> = [
        ["M", "S", "M", "S"],
        ["S", "M", "S", "M"],
        ["M", "M", "S", "S"],
        ["S", "S", "M", "M"],
    ].to_vec();

    let coords = [
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1)
    ];

    options.iter().any(|option| {
        option.iter().enumerate().all(|(i, mas)| {
            mas == &letter_from_grid(grid.clone(), (x as i32 + coords[i].0) as usize, (y as i32 + coords[i].1) as usize)
        })
    })
}

fn letter_from_grid(grid: Vec<Vec<&str>>, x: usize, y: usize) -> &str {
    let empty_vec = vec![];
    grid.get(y).unwrap_or(&empty_vec).get(x).unwrap_or(&"")
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 18);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 9);
    }
}