fn compute_list(list: Vec<(usize, i32)>) -> Vec<i32> {
    let mut new_list = list
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<i32>>();
    new_list.sort();
    new_list
}

fn main() {
    let (left, right): (Vec<_>, Vec<_>) = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);

    let res = compute_list(left)
        .iter()
        .zip(compute_list(right))
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{res}");
}
