use std::collections::{HashMap, HashSet};
use std::ops::Range;

struct Grid {
    x_max: Range<i32>,
    y_max: Range<i32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn compute_all_antinodes(a: Position, other_pos: &[Position], grid: &Grid) -> Vec<Position> {
    

    other_pos
        .iter()
        .flat_map(|b| {
            let mut antinodes = Vec::new();
            let vec = Position {
                x: b.x - a.x,
                y: b.y - a.y,
            };
            let mut start = a;
            while grid.x_max.contains(&start.x) && grid.y_max.contains(&start.y) {
                antinodes.push(start);
                start = Position {
                    x: start.x - vec.x,
                    y: start.y - vec.y,
                };
            }
            start = *b;
            while grid.x_max.contains(&start.x) && grid.y_max.contains(&start.y) {
                antinodes.push(start);
                start = Position {
                    x: start.x + vec.x,
                    y: start.y + vec.y,
                };
            }
            antinodes
        })
        .collect()
}

fn main() {
    let data = include_str!("../input.txt");
    let mut map: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.entry(c).or_default().push(Position {
                x: x as i32,
                y: y as i32,
            });
        }
    }
    let grid = Grid {
        x_max: 0..data.lines().count() as i32,
        y_max: 0..data.lines().last().unwrap().chars().count() as i32,
    };
    for key in map.keys() {
        if *key == '.' {
            continue;
        }
        antinodes.extend(
            map[key]
                .iter()
                .enumerate()
                .map(|(i, x)| compute_all_antinodes(*x, &map[key][i + 1..], &grid)),
        );
    }
    let result: HashSet<&Position> = antinodes.iter().flatten().collect();
    println!("{}", result.len());
}
