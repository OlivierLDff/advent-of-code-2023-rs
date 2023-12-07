advent_of_code::solution!(5);

use anyhow::{Context, Result};

#[derive(Debug)]
struct Mapping {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: u64,
    length: u64,
}

impl Mapping {
    fn in_source_range(&self, value: u64) -> bool {
        self.source <= value && value < self.source + self.length
    }

    pub fn map(&self, value: u64) -> Option<u64> {
        if self.in_source_range(value) {
            Some(self.destination + value - self.source)
        } else {
            None
        }
    }
}

fn parse_mapping(input: &str) -> Result<Mapping> {
    let mut parts = input.split_ascii_whitespace();
    let destination = parts
        .next()
        .context("There should be a destination")?
        .parse()
        .context("The destination should be a number")?;
    let source = parts
        .next()
        .context("There should be a source")?
        .parse()
        .context("The source should be a number")?;
    let length = parts
        .next()
        .context("There should be a length")?
        .parse()
        .context("The length should be a number")?;

    Ok(Mapping {
        source,
        destination,
        length,
    })
}

fn parse_mappings(input: &str, expected_name: &str) -> Result<Vec<Mapping>> {
    let mut parts = input.lines();
    if !parts
        .next()
        .context("There should be a header line")?
        .starts_with(expected_name)
    {
        return Err(anyhow::anyhow!(
            "Expected {} but got {}",
            expected_name,
            input
        ));
    }

    let mut mappings: Result<Vec<_>> = parts.map(parse_mapping).collect();
    if let Ok(mappings) = &mut mappings {
        mappings.sort_by_key(|mapping| mapping.source);
    }
    mappings
}

fn parse_almanac_multi_mapping(parts: &[&str]) -> Result<Vec<Vec<Mapping>>> {
    [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .iter()
    .zip(parts.iter())
    .map(|(expected_name, part)| parse_mappings(*part, *expected_name))
    .collect()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn new_from_str(input: &str) -> Result<Almanac> {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .context("There should be a part for the seeds")?
            .split(": ")
            .nth(1)
            .context("There should be a second part, for the seeds")?
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();
        let mappings = parse_almanac_multi_mapping(&parts.collect::<Vec<_>>())?;
        Ok(Almanac { seeds, mappings })
    }

    fn map_seed(&self, seed: u64) -> u64 {
        self.mappings.iter().fold(seed, |current, mappings| {
            mappings
                .iter()
                .find_map(|mapping| mapping.map(current))
                .unwrap_or(current)
        })
    }

    fn get_closest_seed_location(&self) -> Option<u64> {
        self.seeds.iter().map(|seed| self.map_seed(*seed)).min()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = Almanac::new_from_str(input).unwrap();
    let closest_location = almanac.get_closest_seed_location()?;
    Some(closest_location as u32)
}

fn split_range(range: &Range, source: u64) -> (Option<Range>, Option<Range>) {
    if source > range.start {
        let length = (source - range.start).min(range.length);
        (
            Some(Range {
                start: range.start,
                length,
            }),
            if length == range.length {
                None
            } else {
                Some(Range {
                    start: source,
                    length: range.length - length,
                })
            },
        )
    } else {
        (None, Some(range.clone()))
    }
}

// Get the before, contained, and remaining range for a mapping
fn split_range_for_mapping(
    range: &Range,
    mapping: &Mapping,
) -> (Option<Range>, Option<Range>, Option<Range>) {
    let (before_range, remaining_range) = split_range(&range, mapping.source);

    if let Some(range) = remaining_range {
        let (contained_range, remaining_range) =
            split_range(&range, mapping.source + mapping.length);
        (before_range, contained_range, remaining_range)
    } else {
        (before_range, None, None)
    }
}

fn map_range(range: &Range, mappings: &[Mapping]) -> Vec<Range> {
    // assert!(mappings.is_sorted_by_key(|mapping| mapping.source));

    let mut result = Vec::new();
    let mut current = Some(range.clone());
    for mapping in mappings {
        if let Some(range) = current.take() {
            let (before_range, contained_range, remaining_range) =
                split_range_for_mapping(&range, mapping);

            if let Some(range) = before_range {
                result.push(range);
            }
            if let Some(range) = contained_range {
                result.push(Range {
                    start: mapping
                        .map(range.start)
                        .expect("If we are here, it means we are contained in the mapping"),
                    length: range.length,
                })
            }

            if let Some(range) = remaining_range {
                current = Some(range);
            }
        } else {
            break;
        }
    }
    if let Some(current) = current {
        result.push(current);
    }
    result
}

fn map_range_recursive(range: &Range, mappings: &[Vec<Mapping>]) -> Vec<Range> {
    if mappings.is_empty() {
        vec![range.clone()]
    } else {
        map_range(range, &mappings[0])
            .iter()
            .map(|range| map_range_recursive(range, &mappings[1..]))
            .flatten()
            .collect()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let almanac = Almanac::new_from_str(input).unwrap();

    // seeds are now ranges
    let seeds = almanac
        .seeds
        .chunks(2)
        .map(|w| (w[0], w[1]))
        .map(|(start, length)| {
            if length <= 0 {
                Err(anyhow::anyhow!("Invalid length {}", length))
            } else {
                Ok(Range { start, length })
            }
        })
        .collect::<Result<Vec<_>>>()
        .unwrap();

    seeds
        .iter()
        .map(|range| map_range_recursive(range, &almanac.mappings))
        .flatten()
        .map(|range| range.start as u32)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_split_range() {
        assert_eq!(
            split_range(
                &Range {
                    start: 5,
                    length: 10
                },
                7
            ),
            (
                Some(Range {
                    start: 5,
                    length: 2
                }),
                Some(Range {
                    start: 7,
                    length: 8
                })
            )
        );

        assert_eq!(
            split_range(
                &Range {
                    start: 5,
                    length: 10
                },
                20
            ),
            (
                Some(Range {
                    start: 5,
                    length: 10
                }),
                None
            )
        );

        assert_eq!(
            split_range(
                &Range {
                    start: 5,
                    length: 10
                },
                4
            ),
            (
                None,
                Some(Range {
                    start: 5,
                    length: 10
                })
            )
        );
    }

    #[test]
    fn test_map_range() {
        let range = Range {
            start: 5,
            length: 10,
        };

        let dest_ranges = map_range(
            &range,
            &vec![
                Mapping {
                    source: 3,
                    destination: 103,
                    length: 5,
                },
                Mapping {
                    source: 9,
                    destination: 201,
                    length: 10,
                },
            ],
        );
        assert_eq!(
            dest_ranges,
            vec![
                Range {
                    start: 105,
                    length: 3,
                },
                Range {
                    start: 8,
                    length: 1,
                },
                Range {
                    start: 201,
                    length: 6,
                }
            ]
        )
    }
}
