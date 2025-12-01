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

fn verify_update(
    rules: &HashMap<i32, HashSet<i32>>,
    update: &mut Vec<i32>,
    run: usize,
) -> (Vec<i32>, usize) {
    let mut forbiden: HashSet<i32> = HashSet::new();
    for (index, page) in update.iter().enumerate() {
        if forbiden.contains(page) {
            update.swap(index, index - 1);
            return verify_update(rules, update, run + 1);
        }
        if let Some(new_rules) = rules.get(page) {
            forbiden.extend(new_rules);
        }
    }
    (update.to_vec(), run)
}

fn main() {
    let (rules, mut updates) = parse();
    let result: i32 = updates
        .iter_mut()
        .map(|update| verify_update(&rules, update, 0))
        .map(|(update, run)| {
            if run > 0 {
                *update.get(update.len() / 2).unwrap()
            } else {
                0
            }
        })
        .sum();
    println!("{result:?}");
}
