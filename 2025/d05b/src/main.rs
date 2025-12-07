#[derive(Clone, Copy, Debug, PartialEq)]
enum LimitType{
    Start,
    End
}

#[derive(Clone, Copy, Debug)]
struct Limit{
    value: i64,
    limittype: LimitType
}

fn parse_data(data: &'static str) -> Vec<Limit> {
    let mut whole = data.split("\n\n");
    let mut limits_vec = Vec::new();
    whole
        .next()
        .unwrap()
        .lines()
        .for_each(|l| {
            let mut limits = l.split("-");
            let start = limits.next().unwrap().parse::<i64>().unwrap();
            let end = limits.next().unwrap().parse::<i64>().unwrap();
            limits_vec.push(Limit{value: start, limittype: LimitType::Start});
            limits_vec.push(Limit{value: end, limittype: LimitType::End})
        });
    limits_vec.sort_by(|a, b| {a.value.cmp(&b.value)});
    limits_vec
}

fn count_fresh(limits: Vec<Limit>) -> i64 {
    let mut result = 0;
    let mut start = Limit{value: 0, limittype: LimitType::Start};
    let mut end = Limit{value: 0, limittype: LimitType::End};
    let mut overlaps = 0;
    for limit in &limits{
        match limit.limittype{
            LimitType::Start => {
                if overlaps == 0{
                    start = *limit;
                }
                overlaps +=1;
                if end.value == start.value {
                    start.value += 1;
                }
            }
            LimitType::End=> {
                overlaps -=1;
                if overlaps == 0{
                    result += limit.value - start.value + 1;
                }
                end = *limit;
            }
        }
        if overlaps < 0 {
            println!("bug");
        }
    }
    result 
}

fn main() {
    let ranges = parse_data(include_str!("../input.txt"));
    println!("{}", count_fresh(ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_example() {
        let ranges = parse_data("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n");
        assert_eq!(14, count_fresh(ranges));
    }

    #[test]
    fn test_example() {
        let ranges = parse_data("3-5\n10-14\n16-18\n12-18\n18-19\n18-18\n\n1\n5\n8\n11\n17\n32\n");
        assert_eq!(13, count_fresh(ranges));
    }
}
