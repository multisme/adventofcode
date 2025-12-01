fn is_not_increasing(value: i32, next_value: i32) -> bool {
    value >= next_value || next_value - value > 3
}

fn is_not_decreasing(value: i32, next_value: i32) -> bool {
    value <= next_value || value - next_value > 3
}

fn is_tolerantly_varying(report: &[i32], depth: u8, varying: fn(i32, i32) -> bool) -> bool {
    let size = report.len();
    if size <= 1 {
        return true;
    }
    for i in 0..size - 1 {
        if varying(report[i], report[i + 1]) {
            if depth > 0 {
                return false;
            }
            let mut slice1 = report.to_vec();
            let mut slice2 = slice1.clone();
            slice1.remove(i);
            slice2.remove(i + 1);
            return is_tolerantly_varying(&slice1, depth + 1, varying)
                || is_tolerantly_varying(&slice2, depth + 1, varying);
        }
    }
    true
}

fn prepare_report(report: &[i32]) -> bool {
    is_tolerantly_varying(report, 0, is_not_increasing)
        || is_tolerantly_varying(report, 0, is_not_decreasing)
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
        .filter(|report| prepare_report(report))
        .collect::<Vec<_>>();
    println!("{:?}", safe_reports.len());
}
