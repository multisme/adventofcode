use std::collections::HashMap;
use std::collections::HashSet;

fn parse() -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let data = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    data[0].lines().for_each(|l| {
        let values = l
            .split("|")
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        rules.entry(values[1]).or_default().insert(values[0]);
    });

    let updates: Vec<_> = data[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn verify_update(rules: &HashMap<i32, HashSet<i32>>, update: Vec<i32>) -> bool {
    let mut forbiden: HashSet<i32> = HashSet::new();
    for page in update {
        if forbiden.contains(&page) {
            return false;
        }
        if let Some(new_rules) = rules.get(&page) {
            forbiden.extend(new_rules);
        }
    }
    true
}

fn main() {
    let (rules, updates) = parse();
    let result: i32 = updates
        .into_iter()
        .filter(|update| verify_update(&rules, update.to_vec()))
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum();
    println!("{result:?}");
}
