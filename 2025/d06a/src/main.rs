enum Sign {
    Plus,
    Product,
}

fn parse_data(data: &'static str) -> (Vec<Vec<i64>>, Vec<Sign>) {
    let lines = data.lines();
    let size = lines.clone().count();
    let binding = lines.collect::<Vec<&str>>();
    let (num_str, sign_str) = binding.split_at(size - 1);
    let signs: Vec<Sign> = sign_str[0]
        .split_whitespace()
        .map(|x| match x {
            "+" => Sign::Plus,
            _ => Sign::Product,
        })
        .collect();
    let mut table: Vec<Vec<i64>> = vec![];
    for _column in 0..signs.len() {
        table.push(Vec::new());
    }
    for line in num_str {
        line.split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .enumerate()
            .for_each(|(i, n)| table[i].push(n));
    }
    (table, signs)
}

fn calculate(table: Vec<Vec<i64>>, signs: Vec<Sign>) -> i64 {
    let mut res = 0;
    for (index, sign) in signs.iter().enumerate() {
        match sign {
            Sign::Plus => res += table[index].iter().sum::<i64>(),
            Sign::Product => res += table[index].iter().product::<i64>(),
        }
    }
    res
}

fn main() {
    let (table, signs) = parse_data(include_str!("../input.txt"));
    println!("{}", calculate(table, signs));
}

#[cfg(test)]
mod tests {
    use super::*;
}
