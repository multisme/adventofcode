use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn explore_grid(map: &HashMap<Position, u32>, pos: Position, height: u32) -> u32 {
    match map.get(&pos) {
        Some(new_height) => {
            if *new_height != height {
                return 0;
            } else if height == 9 {
                return 1;
            }
            explore_grid(
                map,
                Position {
                    x: pos.x + 1,
                    y: pos.y,
                },
                height + 1,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                height + 1,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
                height + 1,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                height + 1,
            )
        }
        _ => 0,
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut map: HashMap<Position, u32> = HashMap::new();
    let mut starts: Vec<Position> = Vec::new();
    let mut ends: Vec<Position> = Vec::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(height) = c.to_digit(10) {
                let point = Position {
                    x: x as i32,
                    y: y as i32,
                };
                map.entry(point).or_insert(height);
                if height == 0 {
                    starts.push(point);
                } else if height == 9 {
                    ends.push(point);
                }
            }
        }
    }
    let result: u32 = starts.iter().map(|p| explore_grid(&map, *p, 0)).sum();
    println!("{result}");
}
