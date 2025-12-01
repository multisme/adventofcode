enum Direction {
    North,
    East,
    South,
    West,
}

struct Guard {
    dir: Direction,
    position: usize,
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

    fn step(&mut self, row_size: usize, size: usize) -> Option<usize> {
        match self.dir {
            Direction::North => self.position.checked_sub(row_size),
            Direction::East => Some(self.position + 1),
            Direction::South => {
                let pos = self.position + row_size;
                if pos > size {
                    None
                } else {
                    Some(pos)
                }
            }
            Direction::West => self.position.checked_sub(1),
        }
    }
}

fn main() {
    let data = include_str!("../test.txt");
    let row_size = data.lines().next().unwrap().len() + 1;
    let position = data.chars().position(|x| x == '^').unwrap();
    let dir = Direction::North;
    let size = data.len();

    let mut guard = Guard { dir, position };

    let mut map = data.chars().collect::<Vec<char>>();
    map[position] = 'x';
    while let Some(x) = guard.step(row_size, size) {
        match map[x] {
            '\n' => break,
            '#' => guard.rotate(),
            _ => {
                map[x] = 'X';
                guard.position = x;
            }
        }
    }
    let result = map.iter().filter(|x| **x == 'X').count();
    println!("result: {result}");
}
