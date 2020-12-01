use std::iter::successors;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().map(|&mass| fuel_required(mass)).sum()
}

fn fuel_required(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input.iter().map(|&mass| successors(Some(mass), |&m| needs_fuel(m)).sum::<i32>() - mass).sum()
}

fn needs_fuel(mass: i32) -> Option<i32> {
    let fuel = fuel_required(mass);
    if fuel < 0 { None } else { Some(fuel) }
}

