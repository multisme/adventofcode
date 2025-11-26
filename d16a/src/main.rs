use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Score {
    char: char,
    points: i32,
}

enum Direction {
    Horizontal,
    Vertical,
}

impl Position {
    fn p_move(&self, direction: (i32, i32)) -> Position {
        Position {
            x: self.x + direction.0,
            y: self.y + direction.1,
        }
    }
    fn top(self) -> Position {
        self.p_move((0, 1))
    }
    fn bottom(self) -> Position {
        self.p_move((0, -1))
    }
    fn right(self) -> Position {
        self.p_move((1, 0))
    }
    fn left(self) -> Position {
        self.p_move((-1, 0))
    }
}

fn parse_grid(map_str: &str) -> (usize, usize, Position, Position, Vec<Vec<Score>>) {
    let mut grid = Vec::new();
    let (mut height, mut width) = (0, 0);
    let (mut start, mut end) = (Position{x: 0, y: 0}, Position{x: 0, y: 0});
    map_str.lines().enumerate().for_each(|(y, line)| {
        let mut row: Vec<Score> = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            row.push(Score { char: c, points: 0 });
            width = x;
            if c == 'S' {
                start = Position { x: x as i32, y: y as i32};
            }
            if c == 'E' {
                end = Position { x: x as i32, y: y as i32};
            }
        });
        grid.push(row);
        height = y;
    });
    (width, height, start, end, grid)
}

fn print_grid(width: usize, height: usize, grid: HashMap<Position, Score>) {
    let mut map = vec![vec![0; width + 1]; height + 1];
    grid.iter().for_each(|(p, c)| {
        map[p.y as usize][p.x as usize] = c.points;
    });
    map.iter().for_each(|l| println!("{:05?}", l));
    println!("\n\n");
}

fn map_grid(grid: &mut Vec<Vec<Score>>, pos: Position, direction: Direction, points: i32) {
    match grid[pos.y as usize][pos.x as usize] {
        score @ Score { char: '.', .. } | score @ Score { char: 'S', .. } => {
            if score.points == 0 || score.points > points {
                grid[pos.y as usize][pos.x as usize].points = points; 
                let (v_inc, h_inc) = match direction {
                    Direction::Vertical => (1, 1001),
                    Direction::Horizontal => (1001, 1),
                };
                map_grid(grid, pos.top(), Direction::Vertical, points + v_inc);
                map_grid(grid, pos.bottom(), Direction::Vertical, points + v_inc);
                map_grid(grid, pos.left(), Direction::Horizontal, points + h_inc);
                map_grid(grid, pos.right(), Direction::Horizontal, points + h_inc);
            }
        }
        score @ Score { char: 'E', .. } => {
            if score.points == 0 || score.points > points {
                grid[pos.y as usize][pos.x as usize].points = points; 
            }
        }
        _ => (),
    }
}

fn main() {
    let mut data = include_str!("../input.txt").split("\n\n");
    let (_width, _height, start, end, mut grid) = parse_grid(data.next().unwrap());
    let mut deque: VecDeque<Position> = VecDeque::new();
    deque.push_back(start);
    map_grid(&mut grid, start, Direction::Horizontal, 0);
    //print_grid(width, height, grid.clone());
    let res = grid[end.y as usize][end.x as usize].points;
    println!("{res}");
}
