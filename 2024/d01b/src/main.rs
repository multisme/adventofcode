fn main() {
    let (left, right): (Vec<_>, Vec<_>) = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);

    let left = left.into_iter().map(|(_, value)| value);
    let right = right.into_iter().map(|(_, value)| value);

    let res = left
        .map(|a| right.clone().filter(|x| *x == a).count() as i32 * a)
        .sum::<i32>();
    println!("{res}")
}
