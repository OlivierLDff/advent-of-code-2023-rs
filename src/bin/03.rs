advent_of_code::solution!(3);

use std::collections::HashMap;

use anyhow::{Context, Result};

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct Number {
    value: u32,
    x: usize,
    y: usize,
    len: usize,
}

fn parse_map(input: &str) -> Result<Map> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map.first().context("Missing first row")?.len();
    let height = map.len();

    Ok(Map { map, width, height })
}

fn find_numbers(map: &Map) -> Vec<Number> {
    let mut numbers = Vec::new();

    for y in 0..map.height {
        let mut current_number: Option<Number> = None;
        for x in 0..map.width {
            match map.map[y][x].to_digit(10) {
                Some(d) => match current_number.as_mut() {
                    Some(number) => {
                        number.len += 1;
                        number.value = number.value * 10 + d;
                    }
                    None => {
                        current_number = Some(Number {
                            value: d,
                            x,
                            y,
                            len: 1,
                        })
                    }
                },
                None => {
                    if let Some(current_number) = current_number.take() {
                        numbers.push(current_number);
                    }
                }
            }
        }

        if let Some(current_number) = current_number.take() {
            numbers.push(current_number);
        }
    }

    numbers
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn get_adjacent_symbols(map: &Map, number: &Number) -> Vec<(usize, usize)> {
    let x = number.x;
    let y = number.y;
    let len = number.len;
    let mut symbols = Vec::new();

    let first_x = {
        if x > 0 {
            x - 1
        } else {
            0
        }
    };

    // Scan top and bottom line
    for current_x in first_x..=x + len {
        if current_x >= map.width {
            continue;
        }
        let mut y_to_check = vec![y + 1];
        if y > 0 {
            y_to_check.push(y - 1);
        }

        for current_y in y_to_check {
            if current_y >= map.height {
                continue;
            }
            if is_symbol(map.map[current_y][current_x]) {
                symbols.push((current_x, current_y));
            }
        }
    }

    // Scan left & right columns, ignoring corners
    let mut x_to_check = vec![x + len];
    if x > 0 {
        x_to_check.push(x - 1);
    }

    for current_x in x_to_check {
        if current_x >= map.width {
            continue;
        }
        if is_symbol(map.map[y][current_x]) {
            symbols.push((current_x, y));
        }
    }

    symbols
}

fn find_numbers_with_adjacent_symbol<'a>(map: &Map, numbers: &'a [Number]) -> Vec<&'a Number> {
    numbers
        .iter()
        .filter(|number| !get_adjacent_symbols(map, number).is_empty())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input).ok()?;
    let numbers = find_numbers(&map);
    let numbers = find_numbers_with_adjacent_symbol(&map, &numbers);

    numbers.iter().map(|n| n.value).sum::<u32>().into()
}

fn find_gears_adjacent_numbers<'a>(
    map: &Map,
    numbers: &'a [Number],
) -> HashMap<(usize, usize), Vec<&'a Number>> {
    let mut gears_adjacent_numbers = HashMap::new();

    for number in numbers {
        let adjacent_symbols = get_adjacent_symbols(map, number);
        for (x, y) in adjacent_symbols {
            let c = map.map[y][x];
            if c != '*' {
                continue;
            }

            let adjacent_numbers = gears_adjacent_numbers.entry((x, y)).or_insert(Vec::new());
            adjacent_numbers.push(number);
        }
    }

    gears_adjacent_numbers
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input).ok()?;
    let numbers = find_numbers(&map);
    let gears_adjacent_numbers = find_gears_adjacent_numbers(&map, &numbers);

    gears_adjacent_numbers
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |acc, n| acc * n.value))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(467835));
    }
}
