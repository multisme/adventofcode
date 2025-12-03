use regex::Regex;

#[derive(Debug)]
enum Side{
    Left,
    Right
}

#[derive(Debug)]
struct Rotation{
    side: Side,
    distance: i32
}

fn parse_data(data: &'static str) -> Vec<Rotation>{
    let re = Regex::new("(?ms)^([L,R])([0-9]+)$").unwrap();
    let mut result = vec![];
    for (_, [rot_side, rot_distance]) in re.captures_iter(data).map(|rot| rot.extract()) {
        let side = match rot_side {
            "L" => Side::Left,
            _ => Side::Right
        };
        let distance = rot_distance.parse::<i32>().unwrap();
        result.push(Rotation{side, distance});
    };
    result
}

fn dialing(input: Vec<Rotation>) -> i32 {
    let mut dial: i32 = 50;
    let mut res = 0;

    input.iter().for_each(
        |x| {
            let normalised_distance = x.distance % 100;
            //Counting the number of rounds induced by distances > 100
            res += x.distance / 100;
            let mut click = false;
            match x.side {
                Side::Left => {
                    click = dial > 0;
                    dial -= normalised_distance;
                    if dial < 0{
                        if click {
                            res += 1;
                        }
                        dial += 100;
                    }
                },
                Side::Right => {
                    click = dial < 100;
                    dial += normalised_distance;
                    if dial > 100{
                        if click {
                            res += 1;
                        }
                        dial -= 100;
                    }
                },
            }
                if dial == 0 || dial == 100 {
                    res += 1;
                }
            println!("{} {}, {}", match x.side { Side::Left => "-", Side::Right => "+"},x.distance, res)
        }
    );
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
    fn test_example(){
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(dialing(parse_data(input)), 6);
    }
}
