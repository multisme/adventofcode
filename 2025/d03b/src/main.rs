fn parse_data(data: &'static str) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_battery(bank: &Vec<char>, start: usize, end: usize) -> (usize, char) {
    let (index, &letter) = bank
        .iter()
        .enumerate()
        .skip(start)
        .reduce(|acc, x| match x {
            val if val.1 > acc.1 && x.0 <= end => x,
            _ => acc,
        })
        .unwrap();
    (index, letter)
}

fn process_bank(bank: &Vec<char>, counts: usize) -> u64 {
    let mut end = bank.len() - counts;
    let mut start = 0;
    let mut res = 0;
    for i in 0..counts {
        let (index, letter) = find_battery(bank, start, end);
        res = res * 10 + letter.to_digit(10).unwrap() as u64;
        start = index + 1;
        end += 1;
    }
    res
}

fn process_banks(banks: Vec<Vec<char>>, counts: usize) -> u64 {
    banks.iter().map(|b| process_bank(b, counts)).sum()
}

fn main() {
    let input = parse_data(include_str!("../input.txt"));
    println!("{}", process_banks(input, 12));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = parse_data(include_str!("../input.txt"));
        
        assert_eq!(17376, process_banks(input, 2))
    }

    #[test]
    fn test_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        
        assert_eq!(357, process_banks(parse_data(input), 2));
        assert_eq!(3205, process_banks(parse_data(input), 3))
    }
}
