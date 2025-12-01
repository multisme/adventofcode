use std::collections::HashMap;

#[macro_export]
macro_rules! left {
    () => {};
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn explore_grid(
    map: &mut HashMap<Position, (char, bool)>,
    pos: Position,
    plant: char,
    size: &mut u32,
) -> u32 {
    match map.get_mut(&pos) {
        Some(address) => {
            if address.0 != plant {
                return 1;
            }
            if address.1 {
                return 0;
            }
            *size += 1;
            address.1 = true;
            explore_grid(
                map,
                Position {
                    x: pos.x + 1,
                    y: pos.y,
                },
                plant,
                size,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                plant,
                size,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
                plant,
                size,
            ) + explore_grid(
                map,
                Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                plant,
                size,
            )
        }
        _ => 1,
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut map: HashMap<Position, (char, bool)> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, plot) in line.chars().enumerate() {
            let point = Position {
                x: x as i32,
                y: y as i32,
            };
            map.entry(point).or_insert((plot, false));
        }
    }
    let mut grid = map.clone();
    let result: u32 = map
        .iter()
        .map(|(pos, (plant, _visited))| {
            let mut perimeter = 0;
            let area = explore_grid(&mut grid, *pos, *plant, &mut perimeter);
            area * perimeter
        })
        .sum();
    println!("{result}");
}
