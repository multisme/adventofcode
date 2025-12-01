struct Calibration {
    result: i64,
    values: Vec<i64>,
}

fn join_num(left: i64, right: i64) -> i64 {
    [left.to_string(), right.to_string()]
        .concat()
        .parse()
        .unwrap()
}

fn test_calibration(result: i64, acc: i64, values: &[i64]) -> bool {
    if acc > result || (values.is_empty() && acc != result) {
        return false;
    }
    if values.is_empty() {
        return true;
    }
    test_calibration(result, acc + values[0], &values[1..])
        || test_calibration(result, acc * values[0], &values[1..])
        || test_calibration(result, join_num(acc, values[0]), &values[1..])
}

fn main() {
    let data = include_str!("../input.txt");

    let calibrations_result = data
        .lines()
        .map(|x| {
            let mut line = x.split(": ");
            Calibration {
                result: line.next().unwrap().parse().unwrap(),
                values: line
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            }
        })
        .filter(|x| test_calibration(x.result, x.values[0], &x.values[1..]))
        .fold(0, |acc, x| acc + x.result);
    println!("{calibrations_result}");
}
