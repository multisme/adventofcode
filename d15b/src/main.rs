use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn p_move(self, direction: (i32, i32)) -> Position {
        Position {
            x: self.x + direction.0,
            y: self.y + direction.1,
        }
    }
}

fn parse_grid(map_str: &str) -> (usize, usize, HashMap<Position, char>) {
    let mut grid: HashMap<Position, char> = HashMap::new();
    let (mut height, mut width) = (0, 0);
    map_str.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;
        line.chars().for_each(|c| {
            if c == 'O' {
                grid.insert(Position { x, y: y as i32 }, '[');
                x += 1;
                grid.insert(Position { x, y: y as i32 }, ']');
            } else {
                grid.insert(Position { x, y: y as i32 }, c);
                x += 1;
                if c != '@' {
                    grid.insert(Position { x, y: y as i32 }, c);
                } else {
                    grid.insert(Position { x, y: y as i32 }, '.');
                }
            }
            x += 1;
        });
        height = y;
        width = x as usize;
    });
    (width, height, grid)
}

fn parse_path(path_str: &str) -> String {
    path_str.split_whitespace().collect::<Vec<&str>>().concat()
}

fn move_robot(
    direction: (i32, i32),
    items: &[(Position, char)],
    grid: &mut HashMap<Position, char>,
    to_update: &mut Vec<(Position, char)>,
) -> Vec<bool> {
    items
        .iter()
        .map(|(p, c)| {
            let n = p.p_move(direction);
            let res = grid.get(&n);
           // println!("new {:?}, {:?} {:?} {:?}", p, c, n, res);
            match res {
                Some('.') => {
                    if !to_update.contains(&(n, *c)) {
                        to_update.push((n, *c));
                        to_update.push((*p, '.'));
                    }
                    true
                }
                Some('[') => match direction {
                    (0, _) => {
                        let r = n.p_move((1, 0));
                        match move_robot(direction, &[(n, '['), (r, ']')], grid, to_update)[..] {
                            [true, true] => {
                                if !to_update.contains(&(n, *c)) {
                                    to_update.push((n, *c));
                                    to_update.push((*p, '.'));
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                    _ => match move_robot(direction, &[(n, '[')], grid, to_update)[..] {
                        [true] => {
                            if !to_update.contains(&(n, *c)) {
                                to_update.push((n, *c));
                                to_update.push((*p, '.'));
                            }
                            true
                        }
                        _ => false,
                    },
                },
                Some(']') => match direction {
                    (0, _) => {
                        let l = n.p_move((-1, 0));
                        match move_robot(direction, &[(n, ']'), (l, '[')], grid, to_update)[..] {
                            [true, true] => {
                                if !to_update.contains(&(n, *c)) {
                                    to_update.push((n, *c));
                                    to_update.push((*p, '.'));
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                    _ => match move_robot(direction, &[(n, ']')], grid, to_update)[..] {
                        [true] => {
                            if !to_update.contains(&(n, *c)) {
                                to_update.push((n, *c));
                                to_update.push((*p, '.'));
                            }
                            true
                        }
                        _ => false,
                    },
                },
                _ => false,
            }
        })
        .collect()
}

fn print_grid(width: usize, height: usize, grid: HashMap<Position, char>) {
    let mut map = vec![vec!['.'; width]; height + 1];
    grid.iter().for_each(|(p, c)| {
        map[p.y as usize][p.x as usize] = *c;
    });
    map.iter()
        .for_each(|l| println!("{}", String::from_iter(l)));
    println!("\n\n");
}
fn main() {
    let mut data = include_str!("../input.txt").split("\n\n");
    let (width, height, mut grid) = parse_grid(data.next().unwrap());
    let path = parse_path(data.next().unwrap());
    let mut robot = grid
        .iter_mut()
        .find_map(|x| if *x.1 == '@' { Some((*x.0, '@')) } else { None })
        .unwrap();
    //print_grid(width, height, grid.clone());
    for c in path.chars() {
        let direction = match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        let mut to_update = Vec::new();
        to_update.push((robot.0, '.'));
        if move_robot(direction, &[robot], &mut grid, &mut to_update)
            .iter()
            .any(|x| *x)
        {
            robot.0 = robot.0.p_move(direction);
            to_update.iter().for_each(|(k, v)| {
                grid.insert(*k, *v);
            });
         //   print_grid(width, height, grid.clone());
        }
    }
    let res: i32 = grid
        .iter()
        .filter(|(_k, v)| **v == '[')
        .map(|(k, _v)| {
            let a = k.x + k.y * 100;
            a
        })
        .sum();
    println!("{res}");
}
