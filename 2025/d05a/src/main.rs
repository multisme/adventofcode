use std::ops::Range;

fn parse_data(data: &'static str) -> (Vec<Range<i64>>, Vec<i64>) {
    let mut whole = data.split("\n\n");
    let ranges = whole
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut limits = l.split("-");
            limits.next().unwrap().parse::<i64>().unwrap()
                ..limits.next().unwrap().parse::<i64>().unwrap() + 1
        })
        .collect();
    let ingredients = whole
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    (ranges, ingredients)
}

fn count_fresh(ranges: Vec<Range<i64>>, ingredients: Vec<i64>) -> usize{
    ingredients.iter().filter(|i| {ranges.iter().any(|r| r.contains(i))}).count()
}

fn main(){
    let (ranges, ingredients) = parse_data(include_str!("../input.txt"));
    println!("{}", count_fresh(ranges, ingredients));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (ranges, ingredients) = parse_data("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n");
        assert_eq!(3, count_fresh(ranges, ingredients))
    }
}
