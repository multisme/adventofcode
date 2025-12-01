use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    dir: Direction,
    position: usize,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<char>,
    row_size: usize,
    size: usize,
}

impl Guard {
    fn rotate(&mut self) {
        match self.dir {
            Direction::North => self.dir = Direction::East,
            Direction::East => self.dir = Direction::South,
            Direction::South => self.dir = Direction::West,
            Direction::West => self.dir = Direction::North,
        }
    }
}

impl Map {
    fn step(&self, guard: Guard) -> Option<usize> {
        match guard.dir {
            Direction::North => guard.position.checked_sub(self.row_size),
            Direction::East => Some(guard.position + 1),
            Direction::South => {
                let pos = guard.position + self.row_size;
                if pos >= self.size {
                    None
                } else {
                    Some(pos)
                }
            }
            Direction::West => guard.position.checked_sub(1),
        }
    }
}

fn try_map(mut guard: Guard, map: &Map) -> bool {
    let mut previous_stops = HashSet::new();
    while let Some(x) = map.step(guard) {
        if previous_stops.contains(&(x, guard.dir)) {
            return true;
        }
        match map.grid[x] {
            '\n' => break,
            '#' => {
                previous_stops.insert((x, guard.dir));
                guard.rotate()
            }
            _ => {
                guard.position = x;
            }
        }
    }
    false
}

fn main() {
    let data = include_str!("../input.txt");
    let map = Map {
        grid: data.chars().collect::<Vec<char>>(),
        row_size: data.lines().next().unwrap().len() + 1,
        size: data.len(),
    };

    let guard = Guard {
        dir: Direction::North,
        position: map.grid.iter().position(|&x| x == '^').unwrap(),
    };

    let mut new_obstacle = guard;
    let mut count_loops = 0;
    let mut loops_gen_pos = HashSet::new();
    let mut new_map = map.clone();
    while let Some(x) = map.step(new_obstacle) {
        if loops_gen_pos.contains(&x) {
            new_obstacle.position = x;
            continue;
        }
        match map.grid[x] {
            '\n' => break,
            '#' => new_obstacle.rotate(),
            _ => {
                new_map.grid[x] = '#';
                if try_map(guard, &new_map) {
                    count_loops += 1;
                    loops_gen_pos.insert(x);
                }
                new_map.grid[x] = '.';
                new_obstacle.position = x;
            }
        }
    }
    println!("result: {count_loops}");
}
