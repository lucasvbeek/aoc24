use std::{collections::HashMap, vec};
use rayon::prelude::*;

const INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("Part 1: {}", process_p1(INPUT));
    println!("Part 2: {}", process_p2(INPUT));
}

fn process_p1(input: &str) -> usize {
    let grid = parse(input);
    let (guard_position, guard_direction) = find_guard(&grid).unwrap();
    find_path(&grid, &guard_position, &guard_direction).len()
}

fn process_p2(input: &str) -> usize {
    let grid: Vec<Vec<&str>> = parse(input);
    let (guard_position, guard_direction) = find_guard(&grid).unwrap();

    find_path(&grid, &guard_position, &guard_direction)
        .par_iter()
        .filter(|l| test_path(&grid, l, &guard_position, &guard_direction))
        .count()
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input.split("\n").map(|l| l.split("").collect()).collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

fn find_guard(grid: &Vec<Vec<&str>>) -> Option<((usize, usize), GuardDirection)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let options = HashMap::from([
                ("^", GuardDirection::Up),
                (">", GuardDirection::Right),
                ("v", GuardDirection::Down),
                ("<", GuardDirection::Left),
            ]);
            if options.keys().collect::<Vec<&&str>>().contains(&col) {
                return Some(((x, y), options.get(col).unwrap().clone()));
            }
        }
    }
    None
}


fn find_path(grid: &Vec<Vec<&str>>, starting_position: &(usize, usize), starting_direction: &GuardDirection) -> Vec<(usize, usize)> {
    let mut guard_position = starting_position.clone();
    let mut guard_direction = starting_direction.clone();
    let mut locations: Vec<(usize, usize)> = vec![];
    loop {
        if on_edge(&grid, &guard_position, &guard_direction) {
            break;
        }

        if faces_obstacle(&grid, &guard_position, &guard_direction) {
            guard_direction = next_direction(&guard_direction);
            continue;
        }

        guard_position = next_position(guard_position, &guard_direction);

        if locations.iter().find(|&&l| l == guard_position).is_none() {
            locations.push(guard_position);
        }
    }
    locations
}

fn test_path(grid: &Vec<Vec<&str>>, obstacle_location: &(usize, usize), starting_position: &(usize, usize), starting_direction: &GuardDirection) -> bool {
    let mut test_grid = grid.clone();
    let mut guard_position = starting_position.clone();
    let mut guard_direction = starting_direction.clone();

    test_grid[obstacle_location.1][obstacle_location.0] = "#";
    let mut obstacles: Vec<((usize, usize), GuardDirection)> = vec![];
    loop {    
        if on_edge(&test_grid, &guard_position, &guard_direction) {
            break;
        }

        if faces_obstacle(&test_grid, &guard_position, &guard_direction) {
            if obstacles
                .iter()
                .find(|&&l| l.0 == guard_position && l.1 == guard_direction)
                .is_some()
            {
                return true;
            }

            obstacles.push((guard_position, guard_direction));

            guard_direction = next_direction(&guard_direction);

            continue;
        }

        guard_position = next_position(guard_position, &guard_direction);
    }
    false
}

fn on_edge(grid: &Vec<Vec<&str>>, position: &(usize, usize), direction: &GuardDirection) -> bool {
    match direction {
        GuardDirection::Up => position.1 == 0,
        GuardDirection::Right => position.0 == grid[0].len() - 1,
        GuardDirection::Down => position.1 == grid.len() - 1,
        GuardDirection::Left => position.0 == 0,
    }
}

fn faces_obstacle(
    grid: &Vec<Vec<&str>>,
    position: &(usize, usize),
    direction: &GuardDirection,
) -> bool {
    match direction {
        GuardDirection::Up => grid[position.1 - 1][position.0] == "#",
        GuardDirection::Right => grid[position.1][position.0 + 1] == "#",
        GuardDirection::Down => grid[position.1 + 1][position.0] == "#",
        GuardDirection::Left => grid[position.1][position.0 - 1] == "#",
    }
}

fn next_direction(direction: &GuardDirection) -> GuardDirection {
    match direction {
        GuardDirection::Up => GuardDirection::Right,
        GuardDirection::Right => GuardDirection::Down,
        GuardDirection::Down => GuardDirection::Left,
        GuardDirection::Left => GuardDirection::Up,
    }
}

fn next_position(position: (usize, usize), direction: &GuardDirection) -> (usize, usize) {
    let mut position = position.clone();
    match direction {
        GuardDirection::Up => position.1 -= 1,
        GuardDirection::Right => position.0 += 1,
        GuardDirection::Down => position.1 += 1,
        GuardDirection::Left => position.0 -= 1,
    }
    position
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test.txt");

    #[test]
    fn correct_result_p1() {
        let out = super::process_p1(TEST_INPUT);
        assert_eq!(out, 41);
    }

    #[test]
    fn correct_result_p2() {
        let out = super::process_p2(TEST_INPUT);
        assert_eq!(out, 6);
    }
}
