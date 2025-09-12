use regex::Regex;

fn solve_equation(first: [i32; 2], second: [i32; 2], last: [i32; 2]) -> Option<(i32, i32)> {
    //Compute determinant and then compute answer
    match first[0] * second[1] - first[1] * second[0] {
        0 => None,
        determinant => {
            let quotient_a = (last[0] * second[1] - first[1] * last[1]) / determinant;
            let remainder_a = (last[0] * second[1] - first[1] * last[1]) % determinant;
            let quotient_b = (first[0] * last[1] - last[0] * second[0]) / determinant;
            let remainder_b = (first[0] * last[1] - last[0] * second[0]) % (determinant);
            match (remainder_a, remainder_b) {
                (0, 0) => Some((quotient_a, quotient_b)),
                _ => None,
            }
        }
    }
}

fn compute_path(machine: &str) -> Option<(i32, i32)> {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    let mut specs = re.captures_iter(machine);
    let button_a_str = specs.next().unwrap();
    let button_b_str = specs.next().unwrap();
    let prize_str = specs.next().unwrap();
    let button_a: [i32; 2] = [
        button_a_str[1].parse().unwrap(),
        button_a_str[2].parse().unwrap(),
    ];
    let button_b: [i32; 2] = [
        button_b_str[1].parse().unwrap(),
        button_b_str[2].parse().unwrap(),
    ];
    let prize: [i32; 2] = [prize_str[1].parse().unwrap(), prize_str[2].parse().unwrap()];

    solve_equation(
        [button_a[0], button_b[0]],
        [button_a[1], button_b[1]],
        prize,
    )
}

fn main() {
    let data = include_str!("../input.txt");
    let result = data
        .split("\n\n")
        .fold(0, |acc, machine| match compute_path(machine) {
            Some((push_a @ 1..=100, push_b @ 1..=100)) => acc + (push_a * 3 + push_b),
            _ => acc,
        });
    println!("{result}");
}
