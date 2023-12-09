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
        part_two(input)
    } else {
        part_one(input)
    };

    println!("{res}");
}

fn part_one(input: impl Iterator<Item = String>) -> i32 {
    input.flat_map(|line| part_numbers(line.as_str())).sum()
}

fn part_numbers(line: &str) -> Vec<i32> {
    let (digits, offsets) = line
        .match_indices(|c: char| c.is_ascii_digit())
        .fold((Vec::new(), 0), collect_numbers);

    // let (symbols, indices) = 

    digits
}

fn collect_numbers(
    (acc, pos): (Vec<i32>, usize),
    (index, digit): (usize, &str),
) -> (Vec<i32>, usize) {
    let acc_last = if index == pos + 1 {
        acc[acc.len() - 1]
    } else {
        0
    };

    let res_last = acc_last * 10 + digit.parse::<i32>().unwrap_or(0);
    let mut res = acc;
    res.pop();
    res.push(res_last);

    (res, index)
}

fn part_two(_input: impl Iterator<Item = String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .lines()
            .map(|line| line.to_string());

        let res = part_one(input);
        assert_eq!(res, 4361);
    }
}
