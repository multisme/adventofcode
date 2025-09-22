use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_grid(map_str: &str) -> HashMap<Position, char> {
    let mut grid: HashMap<Position, char> = HashMap::new();
    map_str.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert(
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        });
    });
    grid
}

fn parse_path(path_str: &str) -> String {
    path_str.split_whitespace().collect::<Vec<&str>>().concat()
}

fn move_robot(
    direction: (i32, i32),
    item: (Position, char),
    grid: &mut HashMap<Position, char>,
) -> Option<char> {
    let new_position = Position {
        x: item.0.x + direction.0,
        y: item.0.y + direction.1,
    };
    match grid.get(&new_position) {
        Some('.') => grid.insert(new_position, item.1),
        Some('O') => match move_robot(direction, (new_position, 'O'), grid) {
            Some(_char) => grid.insert(new_position, item.1),
            _ => None,
        },
        _ => None,
    }
}

fn main() {
    let mut data = include_str!("../input.txt").split("\n\n");
    let mut grid = parse_grid(data.next().unwrap());
    let path = parse_path(data.next().unwrap());
    let mut robot = grid
        .iter_mut()
        .find_map(|x| if *x.1 == '@' { Some((*x.0, '@')) } else { None })
        .unwrap();
    for c in path.chars() {
        let direction = match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };

        if let Some(_operation) = move_robot(direction, robot, &mut grid) {
            grid.insert(robot.0, '.');
            robot.0 = Position {
                x: robot.0.x + direction.0,
                y: robot.0.y + direction.1,
            }
        }
    }
    let res: i32 = grid
        .iter()
        .filter(|(_k, v)| **v == 'O')
        .map(|(k, _v)| k.x + k.y * 100)
        .sum();
    println!("{res}");
}
