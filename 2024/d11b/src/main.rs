use std::collections::HashMap;

fn len(value: u64) -> u32 {
    match value {
        0 => 1,
        value => value.ilog10() + 1,
    }
}

fn split(value: u64, len: u32) -> [u64; 2] {
    let div = 10_u64.pow(len / 2);
    [value / div, value % div]
}

fn change(value: u64) -> Vec<u64> {
    if value == 0 {
        return [1].to_vec();
    }
    let size = len(value);
    if size % 2 == 0 {
        split(value, size).to_vec()
    } else {
        [value * 2024].to_vec()
    }
}

fn update(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (&k, &v) in stones.iter() {
        let n_ks = change(k);
        for n_k in n_ks.iter() {
            *new_stones.entry(*n_k).or_default() += v;
        }
    }
    new_stones
}

fn main() {
    let data = include_str!("../input.txt")
        .split_whitespace()
        .map(|v| (v.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();
    let mut stones = data;
    for _i in 0..75 {
        stones = update(stones)
    }
    let res: u64 = stones.values().sum();
    println!("{res}");
}
