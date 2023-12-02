use std::io::{self, BufRead};

fn main() {
    let input = io::stdin().lock();
    println!("{}", first(input));
}

fn first<T: BufRead>(input: T) -> i32 {
    let digits = input
        .lines()
        .filter_map(|res: Result<String, io::Error>| res.ok())
        .map(|line: String| line.chars().filter(|c: &char| c.is_digit(10)).collect());

    let first_last = digits
        .map(|digits: String| vec![digits.chars().next(), digits.chars().last()])
        .map(|first_last: Vec<Option<char>>| {
            first_last
                .into_iter()
                .filter_map(|c: Option<char>| c)
                .collect()
        });

    first_last.flat_map(|s: String| s.parse::<i32>()).sum()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_first() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let reader = BufReader::new(input.as_bytes());
        let res = first(reader);

        assert_eq!(res, 142);
    }
}
