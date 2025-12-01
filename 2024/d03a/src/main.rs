use regex::Regex;

fn main() {
    let memory: &str = include_str!("../input.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res = re
        .captures_iter(memory)
        .map(|c| {
            let (_, [l, r]) = c.extract();
            l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
        })
        .sum::<i32>();
    println!("{res:?}");
}
