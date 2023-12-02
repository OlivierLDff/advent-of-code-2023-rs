advent_of_code::solution!(1);

fn compute_result(first_digit: Option<u32>, last_digit: Option<u32>) -> Option<u32> {
    match (first_digit, last_digit) {
        (Some(first_digit), Some(last_digit)) => Some(first_digit * 10 + last_digit),
        _ => None,
    }
}

fn extract_u32_from_str(s: &str) -> Option<u32> {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in s.chars() {
        if let Some(c) = c.to_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }

    compute_result(first_digit, last_digit)
}

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_u32_with_literal_from_slice(s: &str) -> Option<u32> {
    if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(d);
    }

    for (i, n) in NUMBERS.iter().enumerate() {
        if s.starts_with(n) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn extract_u32_with_literal_from_str(mut s: &str) -> Option<u32> {
    let mut first_digit = None;
    let mut last_digit = None;

    while !s.is_empty() {
        let d = extract_u32_with_literal_from_slice(s);
        s = &s[1..];

        if let Some(d) = d {
            if first_digit.is_none() {
                first_digit = Some(d);
            }
            last_digit = Some(d);
        }
    }

    compute_result(first_digit, last_digit)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(extract_u32_from_str).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(extract_u32_with_literal_from_str).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_u32_from_str() {
        assert_eq!(extract_u32_from_str("1abc2"), Some(12));
        assert_eq!(extract_u32_from_str("a1b2c3d4e5f"), Some(15));
        assert_eq!(extract_u32_from_str("treb7uchet"), Some(77));
    }

    #[test]
    fn test_extract_u32_with_literal_from_str() {
        assert_eq!(extract_u32_with_literal_from_str("two1nine"), Some(29));
        assert_eq!(extract_u32_with_literal_from_str("eightwothree"), Some(83));
        assert_eq!(extract_u32_with_literal_from_str("zoneight234"), Some(14));
    }

    #[test]
    fn test_part_one() {
        let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part_one(example);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let example = "two1nine
    eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let result = part_two(example);
        assert_eq!(result, Some(281));
    }
}
