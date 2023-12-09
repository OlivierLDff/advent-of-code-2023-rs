use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

fn parse_map(input: &str) -> Map {
    let mut parts = input.split("\n\n");
    let instructions = parts
        .next()
        .expect("There should be directions LRLLRLRL")
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let nodes = parts
        .next()
        .expect("There should be nodes")
        .lines()
        .map(|line| {
            let mut parts = line.split(" = ");
            let key = parts.next().expect("There should be a key");
            let value = parts.next().expect("There should be a value");
            let mut parts = value.split(", ");
            let left = parts
                .next()
                .expect("There should be a left")
                .chars()
                .skip_while(|c| !c.is_alphanumeric())
                .take_while(|c| c.is_alphanumeric())
                .collect();
            let right = parts
                .next()
                .expect("There should be a right")
                .chars()
                .skip_while(|c| !c.is_alphanumeric())
                .take_while(|c| c.is_alphanumeric())
                .collect();
            (key.to_string(), Node { left, right })
        })
        .collect();

    Map {
        instructions,
        nodes,
    }
}

fn count_steps_to_node(map: &Map, from: &str, to: &str) -> u32 {
    let mut steps = 0;
    let mut current_node = from;
    loop {
        if current_node == to {
            break;
        }
        for direction in &map.instructions {
            let node = &map
                .nodes
                .get(current_node)
                .unwrap_or_else(|| panic!("There should be a node for {}", current_node));
            match direction {
                Direction::Left => current_node = &node.left,
                Direction::Right => current_node = &node.right,
            }
            steps += 1;
        }
    }

    steps
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    Some(count_steps_to_node(&map, "AAA", "ZZZ"))
}

fn lcm(nums: &[u64]) -> u64 {
    nums.iter().fold(nums[0], |a, b| num::integer::lcm(a, *b))
}

fn count_steps_to_node_multi(map: &Map, start: char, end: char) -> u64 {
    let mut steps = 0;
    let mut current_nodes = map
        .nodes
        .keys()
        .filter(|k| k.ends_with(start))
        .collect::<Vec<_>>();

    let mut reached_nodes_steps = Vec::new();

    loop {
        for direction in &map.instructions {
            for current_node in &mut current_nodes {
                let node = &map
                    .nodes
                    .get(*current_node)
                    .unwrap_or_else(|| panic!("There should be a node for {}", current_node));

                *current_node = match direction {
                    Direction::Left => &node.left,
                    Direction::Right => &node.right,
                }
            }

            steps += 1;

            let node_dones = current_nodes
                .iter()
                .filter(|n| n.ends_with(end))
                .copied()
                .collect::<Vec<_>>();

            for node in node_dones {
                reached_nodes_steps.push(steps as u64);
                let idx = current_nodes.iter().position(|n| *n == node).unwrap();
                current_nodes.remove(idx);
            }

            if current_nodes.is_empty() {
                return lcm(&reached_nodes_steps);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input);
    Some(count_steps_to_node_multi(&map, 'A', 'Z'))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE), Some(2));
        assert_eq!(part_one(EXAMPLE2), Some(6));
    }

    static EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE3);
        assert_eq!(result, Some(6));
    }
}
