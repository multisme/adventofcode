fn parse_data(data: &'static str) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_battery(bank: &Vec<char>) -> (char, char) {
    let bank_size = bank.len();
    let (index, first) = bank
        .iter()
        .enumerate()
        .reduce(|acc, x| match x {
            val if val.1 > acc.1 && val.0 < bank_size - 1 => x,
            _ => acc,
        })
        .unwrap();
    let (_, second) = bank
        .iter()
        .enumerate()
        .skip(index + 1)
        .reduce(|acc, x| match x {
            val if val.1 > acc.1 => x,
            _ => acc,
        })
        .unwrap();

    (*first, *second)
}

fn process_banks(banks: Vec<Vec<char>>) -> u32 {
    banks
        .iter()
        .map(find_battery)
        .map(|(first, second)| {
            let res = first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap();
            println!("{first}{second}");
            res
        })
        .sum()
}

fn main() {
    let input = parse_data(include_str!("../input.txt"));
    println!("{}", process_banks(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

        assert_eq!(357, process_banks(parse_data(input)))
    }
}
