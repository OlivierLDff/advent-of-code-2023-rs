advent_of_code::solution!(9);

fn parse_histories(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse()
                        .unwrap_or_else(|_| panic!("Invalid number: {}", n))
                })
                .collect()
        })
        .collect()
}

fn create_sub_histories(history: &[i64]) -> Vec<Vec<i64>> {
    let mut sub_histories: Vec<Vec<i64>> = Vec::new();
    sub_histories.push(history.to_vec());

    while !sub_histories
        .last()
        .expect("There should be a last history")
        .iter()
        .all(|n| *n == 0)
    {
        let last = sub_histories.last().unwrap();
        sub_histories.push(last.windows(2).map(|w| w[1] - w[0]).collect());
    }

    sub_histories
}

fn extrapolate_next_value(history: &[i64]) -> i64 {
    create_sub_histories(history)
        .iter()
        .rev()
        .skip(1)
        .fold(0, |acc, x| {
            x.last().expect("There should be a last number") + acc
        })
}

fn extrapolate_previous_value(history: &[i64]) -> i64 {
    create_sub_histories(history)
        .iter()
        .rev()
        .skip(1)
        .fold(0, |acc, x| {
            x.first().expect("There should be a first number") - acc
        })
}

pub fn part_one(input: &str) -> Option<i64> {
    let histories = parse_histories(input);
    Some(
        histories
            .iter()
            .map(|history| extrapolate_next_value(history))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let histories = parse_histories(input);
    Some(
        histories
            .iter()
            .map(|history| extrapolate_previous_value(history))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(2));
    }
}
