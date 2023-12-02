use regex::Regex;
use std::{cmp, collections::HashMap};

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let mut split_game = game.split(": ");
            let _ = split_game.next().expect("String conforms to format");

            // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let rounds_string = split_game.next().expect("String conforms to format");

            power(minimum_cubes(rounds_string))
        })
        .sum()
}

fn minimum_cubes(round: &str) -> HashMap<&str, u32> {
    let mut minimum = HashMap::from([("red", 1), ("green", 1), ("blue", 1)]);

    let pattern = Regex::new(r"(,|;)\s").unwrap();

    pattern.split(round).for_each(|string| {
        let mut split = string.split(" ");
        let number = split
            .next()
            .expect("This is the number")
            .parse::<u32>()
            .expect("the converted number");
        let color = split.next().expect("this is the color");

        let current_amount = minimum[color];

        minimum.insert(color, cmp::max(current_amount, number));
    });

    minimum
}

fn power(hash: HashMap<&str, u32>) -> u32 {
    return hash
        .values()
        .fold(0, |accumulator, value| match accumulator {
            0 => *value,
            _ => accumulator * value,
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", HashMap::from([("red", 4), ("green", 2), ("blue", 6)]))] // Game 1
    #[test_case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", HashMap::from([("red", 1), ("green", 3), ("blue", 4)]))] // Game 2
    #[test_case(
        "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        HashMap::from([("red", 20), ("green", 13), ("blue", 6)])
    )] // Game 3
    #[test_case(
        "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        HashMap::from([("red", 14), ("green", 3), ("blue", 15)])
    )] // Game 4
    #[test_case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", HashMap::from([("red", 6), ("green", 3), ("blue", 2)]))] // Game 5
    fn minimum_cubes_test(x: &str, y: HashMap<&str, u32>) {
        let result = minimum_cubes(x);
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
        assert_eq!(result, 2286)
    }
}
