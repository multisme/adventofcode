use regex::Regex;

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    side: Side,
    distance: i32,
}

fn parse_data(data: &'static str) -> Vec<Rotation> {
    let re = Regex::new("(?ms)^([L,R])([0-9]+)$").unwrap();
    let mut result = vec![];
    for (_, [rot_side, rot_distance]) in re.captures_iter(data).map(|rot| rot.extract()) {
        let side = match rot_side {
            "L" => Side::Left,
            _ => Side::Right,
        };
        let distance = rot_distance.parse::<i32>().unwrap();
        let distance = distance % 100;
        result.push(Rotation { side, distance });
    }
    result
}

fn dialing(input: Vec<Rotation>) -> usize {
    let mut dial: i32 = 50;
    let mut res = 0;
    input.iter().for_each(|x| {
        match x.side {
            Side::Left => dial -= x.distance,
            Side::Right => dial += x.distance,
        }
        dial %= 100;
        if dial == 0 {
            res += 1
        };
    });
    res
}

fn main() {
    let input = parse_data(include_str!("../input.txt"));
    println!("{}", dialing(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(dialing(parse_data(input)), 3);
    }
}
