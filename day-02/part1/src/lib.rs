use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let mut split_game = game.split(": ");
            // Game 1
            let game_string = split_game.next().expect("String conforms to format");
            let game_number = get_game_number(game_string);

            // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let rounds_string = split_game.next().expect("String conforms to format");

            let mut rounds = rounds_string.split("; ");

            match rounds.all(is_round_valid) {
                true => game_number,
                false => 0,
            }
        })
        .sum()
}

fn get_game_number(game_string: &str) -> u32 {
    game_string
        .replace("Game ", "")
        .parse::<u32>()
        .expect("valid number")
}

fn is_round_valid(round: &str) -> bool {
    let cubes_in_bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    round.split(", ").all(|string| {
        let mut split = string.split(" ");
        let number = split
            .next()
            .expect("This is the number")
            .parse::<u32>()
            .expect("the converted number");
        let color = split.next().expect("this is the color");

        cubes_in_bag[color] >= number
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Game 1", 1)]
    #[test_case("Game 2", 2)]
    #[test_case("Game 3", 3)]
    #[test_case("Game 10", 10)]
    fn get_game_number_test(x: &str, y: u32) {
        let result = get_game_number(x);
        assert_eq!(result, y);
    }

    #[test_case("3 blue, 4 red", true)] // Game 1
    #[test_case("1 red, 2 green, 6 blue", true)] // Game 1
    #[test_case("2 green", true)] // Game 1
    #[test_case("1 blue, 2 green", true)] // Game 2
    #[test_case("3 green, 4 blue, 1 red", true)] // Game 2
    #[test_case("1 green, 1 blue", true)] // Game 2
    #[test_case("8 green, 6 blue, 20 red", false)] // Game 3
    #[test_case("5 blue, 4 red, 13 green", true)] // Game 3
    #[test_case("5 green, 1 red", true)] // Game 3
    #[test_case("1 green, 3 red, 6 blue", true)] // Game 4
    #[test_case("3 green, 6 red", true)] // Game 4
    #[test_case("3 green, 15 blue, 14 red", false)] // Game 4
    #[test_case("6 red, 1 blue, 3 green", true)] // Game 5
    #[test_case("2 blue, 1 red, 2 green", true)] // Game 5
    fn is_round_valid_test(x: &str, y: bool) {
        let result = is_round_valid(x);
        assert_eq!(result, y);
    }

    #[test]
    fn process_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!(result, 8)
    }
}
