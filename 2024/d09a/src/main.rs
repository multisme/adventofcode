use std::collections::VecDeque;

fn main() {
    let data = include_str!("../input.txt");

    let disk_map = data
        .chars()
        .take(data.len() - 1)
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut memory = Vec::with_capacity(disk_map.iter().sum());
    let mut indice = -1;
    let mut count = 0;
    for value in disk_map.iter() {
        indice = if indice == -1 {
            count
        } else {
            count += 1;
            -1
        };
        for _i in 0..*value {
            memory.push(indice);
        }
    }
    let mut stack: VecDeque<&i64> =
        VecDeque::from(memory.iter().filter(|x| **x != -1).collect::<Vec<_>>());
    let result: i64 = memory
        .iter()
        .map(|x| {
            if *x != -1 {
                match stack.pop_front() {
                    Some(v) => *v,
                    _ => 0,
                }
            } else {
                match stack.pop_back() {
                    Some(v) => *v,
                    _ => 0,
                }
            }
        })
        .enumerate()
        .map(|(a, b)| a as i64 * b)
        .sum();
    println!("{result:?}");
}
