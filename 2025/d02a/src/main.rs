use std::ops::Range;

fn parse_data(data: &'static str) -> Vec<Range<i64>> {
    data.split(",")
        .map(|x| {
            let mut values = x.trim().split('-');
            Range {
                start: values.next().unwrap().parse::<i64>().unwrap(),
                end: values.next().unwrap().parse::<i64>().unwrap() + 1,
            }
        })
        .collect::<Vec<Range<i64>>>()
}

fn filter_id(range: Range<i64>) -> Vec<i64> {
    let mut invalids = vec![];
    for i in range {
        let res = i.to_string();
        let length = res.chars().count();
        if length % 2 == 1 {
            continue;
        }
        let (first, second) = res.split_at(length / 2);
        if first == second {
            invalids.push(i)
        }
    }
    invalids
}

fn process_ids(ids_ranges: Vec<Range<i64>>) -> i64 {
    ids_ranges
        .iter()
        .flat_map(|r| filter_id(r.clone()))
        .sum::<i64>()
}

fn main() {
    let input = parse_data(include_str!("../input.txt"));
    println!("{}", process_ids(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(1227775554, process_ids(parse_data(input)))
    }
}
