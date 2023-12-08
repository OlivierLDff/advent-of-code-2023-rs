advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("There should be a time line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Time should be a number"))
        .collect::<Vec<u64>>();
    let distances = lines
        .next()
        .expect("There should be a distance line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Distance should be a number"))
        .collect::<Vec<u64>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            record: *distance,
        })
        .collect()
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .expect("There should be a time line")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("Time should be a number");
    let distance = lines
        .next()
        .expect("There should be a time line")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("Time should be a number");
    Race {
        time: time,
        record: distance,
    }
}

fn compute_all_distances(time: u64) -> Vec<u64> {
    (0..=time)
        .map(|press_time| {
            let speed = press_time * 1;
            let remaining_time = time - press_time;
            remaining_time * speed
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_input(input);
    Some(
        races
            .iter()
            .map(|race| {
                compute_all_distances(race.time)
                    .iter()
                    .filter(|distance| **distance > race.record)
                    .count() as u32
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let race = parse_input2(input);
    Some(
        compute_all_distances(race.time)
            .iter()
            .filter(|distance| **distance > race.record)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_compute_all_distance() {
        assert_eq!(compute_all_distances(7), vec![0, 6, 10, 12, 12, 10, 6, 0]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(71503));
    }
}
