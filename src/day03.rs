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
    // input.flat_map(|line| part_numbers(line.as_str())).sum()
    input.fold(("".to_string(), 0), part_numbers).1
}

fn part_numbers((prev, acc): (String, i32), line: String) -> (String, i32) {
    let (pos_num, _) = line
        .match_indices(|c: char| c.is_ascii_digit())
        .fold((Vec::new(), 0), collect_numbers);

    let (symbols, _): (Vec<usize>, Vec<&str>) = line
        .match_indices(|c: char| !(c == '.' || c.is_ascii_digit()))
        .unzip();

    let (symbols_prev, _): (Vec<usize>, Vec<&str>) = prev
        .match_indices(|c: char| !(c == '.' || c.is_ascii_digit()))
        .unzip();

    let (_, numbers): (Vec<usize>, Vec<i32>) = pos_num
        .into_iter()
        .filter(|(pos, num)| {
            has_symbol(*pos, *num, &symbols) || has_symbol(*pos, *num, &symbols_prev)
        })
        .unzip();

    let (pos_num_prev, _) = prev
        .match_indices(|c: char| c.is_ascii_digit())
        .fold((Vec::new(), 0), collect_numbers);

    let (_, numbers_prev): (Vec<usize>, Vec<i32>) = pos_num_prev
        .into_iter()
        .filter(|(pos, num)| has_symbol(*pos, *num, &symbols))
        .unzip();

    (
        line,
        acc + numbers.iter().sum::<i32>() + numbers_prev.iter().sum::<i32>(),
    )
}

fn collect_numbers(
    (acc, pos): (Vec<(usize, i32)>, usize),
    (index, digit): (usize, &str),
) -> (Vec<(usize, i32)>, usize) {
    let mut res = acc.clone();
    let (pos_last, num_last) = if index > pos + 1 {
        (index, 0)
    } else {
        res.pop().unwrap_or((index, 0))
    };

    let res_last = num_last * 10 + digit.parse::<i32>().unwrap_or(0);
    res.push((pos_last, res_last));

    (res, index)
}

fn has_symbol(position: usize, number: i32, symbols: &[usize]) -> bool {
    let start = position.saturating_sub(1) as u32;
    let end = (position as u32 + number.ilog10()).saturating_add(1);
    (start..=end).any(|index| symbols.contains(&(index as usize)))
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
