use std::collections::HashMap;

fn windows_with_step<T>(slice: &[T], size: usize, step: usize) -> impl Iterator<Item = &[T]> {
    (0..)
        .map(move |i| i * step)
        .take_while(move |&start| start + size <= slice.len())
        .map(move |start| &slice[start..start + size])
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn top(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn bottom(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn top_left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    fn top_right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    fn bottom_left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn bottom_right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn explore_grid(
    map: &mut HashMap<Position, (char, bool)>,
    pos: Position,
    plant: char,
    (size, corners, adjacent): (&mut u32, &mut u32, bool),
) -> u32 {
    match map.get_mut(&pos) {
        Some((plot, visited)) => {
            if *plot != plant {
                return 1;
            }

            if *visited || !adjacent {
                return 0;
            }

            *visited = true;
            *size += 1;
            let around = &[
                explore_grid(map, pos.left(), plant, (size, corners, true)),
                explore_grid(map, pos.top_left(), plant, (size, corners, false)),
                explore_grid(map, pos.top(), plant, (size, corners, true)),
                explore_grid(map, pos.top_right(), plant, (size, corners, false)),
                explore_grid(map, pos.right(), plant, (size, corners, true)),
                explore_grid(map, pos.bottom_right(), plant, (size, corners, false)),
                explore_grid(map, pos.bottom(), plant, (size, corners, true)),
                explore_grid(map, pos.bottom_left(), plant, (size, corners, false)),
            ];

            let to_analyze = around
                .iter()
                .cycle()
                .take(around.len() + 2)
                .collect::<Vec<_>>();

            for win in windows_with_step(&to_analyze, 3, 2) {
                match *win {
                    [&1, _, &1] => *corners += 1,
                    [&0, &1, &0] => *corners += 1,
                    _ => {}
                }
            }
            0
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
            let mut corners = 0;
            explore_grid(
                &mut grid,
                *pos,
                *plant,
                (&mut perimeter, &mut corners, true),
            );
            corners * perimeter
        })
        .sum();
    println!("{result}");
}
