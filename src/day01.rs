use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let input = io::stdin().lock().lines().map_while(Result::ok);

    let arg: u8 = env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse()
        .unwrap_or(1);

    let res = if arg != 1 {
        second(input)
    } else {
        first(input)
    };

    println!("{res}");
}

fn first(input: impl Iterator<Item = String>) -> i32 {
    let patterns = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    first_last(input, &patterns)
        .filter_map(|number| number.parse::<i32>().ok())
        .sum()
}

fn first_last<'a>(
    input: impl Iterator<Item = String> + 'a,
    patterns: &'a [&'a str],
) -> impl Iterator<Item = String> + 'a {
    input
        .map(move |line| (first_match(&line, patterns), last_match(&line, patterns)))
        .filter_map(|first_last| {
            if let (Some(first), Some(last)) = first_last {
                Some(format!("{first}{last}"))
            } else {
                None
            }
        })
}

fn first_match(line: &str, patterns: &[&str]) -> Option<String> {
    patterns
        .iter()
        .filter_map(|pattern| line.match_indices(pattern).next())
        .min_by_key(|(index, _)| *index)
        .map(|(_, pattern)| pattern.to_string())
}

fn last_match(line: &str, patterns: &[&str]) -> Option<String> {
    patterns
        .iter()
        .filter_map(|pattern| line.rmatch_indices(pattern).next())
        .max_by_key(|(index, _)| *index)
        .map(|(_, pattern)| pattern.to_string())
}

fn second(input: impl Iterator<Item = String>) -> i32 {
    let patterns = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    first_last(input, &patterns)
        .map(substitute)
        .filter_map(|number| number.parse::<i32>().ok())
        .sum()
}

fn substitute(line: String) -> String {
    let substitutes = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    substitutes
        .iter()
        .fold(line, |res, (word, digit)| res.replace(word, digit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .lines()
            .map(|line| line.to_string());

        let res = first(input);
        assert_eq!(res, 142);
    }

    #[test]
    fn test_second() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .lines()
            .map(|line| line.to_string());

        let res = second(input);
        assert_eq!(res, 281);
    }
}
