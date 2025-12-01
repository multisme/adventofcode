fn is_safely_increasing(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|pair| pair[0] < pair[1] && pair[1] - pair[0] <= 3)
}

fn is_safely_decreasing(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && pair[0] - pair[1] <= 3)
}

fn main() {
    let reports = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports = reports
        .iter()
        .filter(|report| is_safely_decreasing(report) || is_safely_increasing(report))
        .collect::<Vec<_>>();
    println!("{:?}", safe_reports.len())
}
