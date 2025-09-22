use regex::Regex;

const GRID_LENGTH: i32 = 101;
const GRID_HEIGHT: i32 = 103;
const EXPECT_TRUNK_SIZE: u32 = 10;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Position,
}

impl Robot {
    pub fn moving(&mut self) {
        self.position.x = match self.position.x + self.velocity.x {
            n if n < 0 => GRID_LENGTH + n,
            n if n >= GRID_LENGTH => n - GRID_LENGTH,
            n => n,
        };
        self.position.y = match self.position.y + self.velocity.y {
            n if n < 0 => GRID_HEIGHT + n,
            n if n >= GRID_HEIGHT => n - GRID_HEIGHT,
            n => n,
        };
    }

    pub fn quadrant(&self) -> Option<usize> {
        let h_middle: i32 = GRID_HEIGHT / 2;
        let v_middle: i32 = GRID_LENGTH / 2;
        match (self.position.x, self.position.y) {
            (l, h) if (0..h_middle).contains(&l) && (0..v_middle).contains(&h) => Some(0),
            (l, h) if (h_middle + 1..GRID_LENGTH).contains(&l) && (0..v_middle).contains(&h) => {
                Some(1)
            }
            (l, h) if (0..h_middle).contains(&l) && (v_middle + 1..GRID_HEIGHT).contains(&h) => {
                Some(2)
            }
            (l, h)
                if (h_middle + 1..GRID_LENGTH).contains(&l)
                    && (v_middle + 1..GRID_HEIGHT).contains(&h) =>
            {
                Some(3)
            }
            (_, _) => None,
        }
    }
}

fn compute_safety(robots: &[Robot]) -> usize {
    robots
        .iter()
        .fold(vec![0; 4], |mut data, r| {
            if let Some(v) = r.quadrant() {
                data[v] += 1
            };
            data
        })
        .iter()
        .product()
}

fn print_map(robots: &[Robot]) {
    let mut map = vec![vec!['.'; GRID_LENGTH as usize]; GRID_HEIGHT as usize];
    robots.iter().for_each(|r| {
        map[r.position.y as usize][r.position.x as usize] = '*';
    });
    map.iter()
        .for_each(|l| println!("{:?}\n", l.iter().collect::<String>()));
}

fn main() {
    let re = Regex::new(r"(?m)^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    let mut seconds = 0;
    let mut counts = Vec::new();

    for (_, [p_x, p_y, v_x, v_y]) in re
        .captures_iter(include_str!("../input.txt"))
        .map(|c| c.extract())
    {
        robots.push(Robot {
            position: Position {
                x: p_x.parse::<i32>().unwrap(),
                y: p_y.parse::<i32>().unwrap(),
            },
            velocity: Position {
                x: v_x.parse::<i32>().unwrap(),
                y: v_y.parse::<i32>().unwrap(),
            },
        })
    }
    loop {
        seconds += 1;
        robots.iter_mut().for_each(|r| r.moving());
        let mut levels_x: [u32; GRID_LENGTH as usize] = [0; 101];
        let mut levels_y: [u32; GRID_HEIGHT as usize] = [0; 103];
        robots.iter().for_each(|r| {
            levels_x[r.position.x as usize] += 1;
            levels_y[r.position.y as usize] += 1;
        });
        if levels_x.iter().filter(|l| **l > EXPECT_TRUNK_SIZE).count() > 14
            && levels_y.iter().filter(|l| **l > EXPECT_TRUNK_SIZE).count() > 14
        {
            print_map(&robots);
            counts.push((seconds, compute_safety(&robots)));
        }
        if seconds > (GRID_LENGTH * GRID_HEIGHT) {
            break;
        }
    }
    counts.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", counts);
}
