use regex::Regex;

fn memory_compute(memory_line: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(memory_line)
        .map(|c| {
            let (_, [l, r]) = c.extract();
            l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

fn main() {
    let binding = "do()".to_string() + include_str!("../input.txt");
    let res = binding
        .split("do()")
        .map(|d| d.split("don't()").next().unwrap())
        .fold(0, |acc, line| acc + memory_compute(line));
    println!("{res:?}");
}
