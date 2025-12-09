fn parse_data(data: &'static str) -> Vec<Vec<char>> {
    data.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn get_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn get_number(table: &[Vec<char>], index: usize, size: usize) -> Option<u32> {
    let mut num: Option<u32> = None;
    for t in table.iter().take(size) {
        num = match get_digit(t[index]) {
            Some(d) => Some(num.unwrap_or(0) * 10 + d),
            None => num,
        };
    }
    num
}

fn calculate(table: Vec<Vec<char>>) -> u64 {
    let size = table.len();
    let mut column_total = 0;
    let mut res = 0;
    let mut sign = ' ';
    for (index, c) in table[size - 1].clone().into_iter().enumerate() {
        // We check the sign and decide what is the starting point from here
        (sign, column_total) = match c {
            '+' => ('+', 0),
            '*' => ('*', 1),
            _ => (sign, column_total),
        };
        match (sign, get_number(&table, index, size)) {
            (_, None) => res += column_total,
            ('+', Some(num)) => column_total += num as u64,
            ('*', Some(num)) => column_total *= num as u64,
            _ => (),
        }
    }
    res + column_total
}

fn main() {
    let res = calculate(parse_data(include_str!("../input.txt")));
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(3263827, calculate(parse_data(input)));
    }
}
