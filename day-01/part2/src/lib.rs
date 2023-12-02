const RADIX: u32 = 10;
pub fn process(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let replaced = replace_alpha_with_decimal(line);
            first_and_last_numbers(&replaced)
        })
        .sum();
}

fn replace_alpha_with_decimal(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "f4ur")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne")
}

fn first_and_last_numbers(line: &str) -> u32 {
    let mut digits = line
        .chars()
        .filter_map(|character| character.to_digit(RADIX));

    let first = digits.next().expect("Always exists");

    match digits.last() {
        Some(number) => format!("{first}{number}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("valid number")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine", "t2o1n9ne")]
    #[test_case("eightwothree", "ei8ht2oth3ee")]
    #[test_case("abcone2threexyz", "abco1e2th3eexyz")]
    #[test_case("xtwone3four", "xt2o1e3f4ur")]
    #[test_case("4nineeightseven2", "4n9neei8htse7en2")]
    #[test_case("zoneight234", "zo1ei8ht234")]
    #[test_case("7pqrstsixteen", "7pqrsts6xteen")]
    fn replace_alpha_with_decimal_test(x: &str, y: &str) {
        let result = replace_alpha_with_decimal(x);
        assert_eq!(result, y);
    }

    #[test_case("t2o1n9ne", 29)]
    #[test_case("ei8ht2oth3ee", 83)]
    #[test_case("abco1e2th3eexyz", 13)]
    #[test_case("xt2o1e3f4ur", 24)]
    #[test_case("4n9neei8htse7en2", 42)]
    #[test_case("zo1ei8ht234", 14)]
    #[test_case("7pqrsts6xteen", 76)]
    fn first_and_last_numbers_test(x: &str, y: u32) {
        let result = first_and_last_numbers(x);
        assert_eq!(result, y);
    }

    #[test]
    fn process_test() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = process(input);
        assert_eq!(result, 281)
    }
}
