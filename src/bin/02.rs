advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

fn parse_game_set(input: &str) -> Option<GameSet> {
    input
        .split(',')
        .try_fold(GameSet::default(), |mut set, part| {
            let mut part = part.trim().split(' ');
            let count = part.next()?.parse::<u32>().ok()?;
            let color = part.next()?.trim();

            match color {
                "red" => set.red += count,
                "green" => set.green += count,
                "blue" => set.blue += count,
                _ => (),
            }

            Some(set)
        })
}

fn parse_game(input: &str) -> Option<Game> {
    let mut parts = input.split(':');
    let id = parts
        .next()?
        .trim()
        .split(' ')
        .nth(1)?
        .parse::<u32>()
        .ok()?;

    let sets = parts
        .next()?
        .trim()
        .split(';')
        .map(parse_game_set)
        .collect::<Option<Vec<_>>>()?;

    Some(Game { id, sets })
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().filter_map(parse_game).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input);
    let possible_games = games.iter().filter(|g| {
        g.sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
    });

    Some(possible_games.map(|g| g.id).sum())
}

fn find_minimum_game_set(game: &Game) -> GameSet {
    game.sets.iter().fold(
        GameSet {
            red: 0,
            green: 0,
            blue: 0,
        },
        {
            |acc, set| GameSet {
                red: acc.red.max(set.red),
                green: acc.green.max(set.green),
                blue: acc.blue.max(set.blue),
            }
        },
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input);
    Some(
        games
            .iter()
            .map(find_minimum_game_set)
            .map(|s| s.red * s.green * s.blue)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, Some(2286));
    }
}
