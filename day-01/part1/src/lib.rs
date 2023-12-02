use std::u32;

const RADIX: u32 = 10;
pub fn process(input: &str) -> u32 {
    input.lines().map(first_and_last_numbers).sum()
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

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn first_and_last_numbers_test(x: &str, y: u32) {
        let result = first_and_last_numbers(x);
        assert_eq!(result, y);
    }

    #[test]
    fn process_test() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let result = process(input);
        assert_eq!(result, 142)
    }
}
