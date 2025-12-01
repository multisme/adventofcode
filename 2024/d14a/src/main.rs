use regex::Regex;

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
    pub fn moving(&mut self, height: i32, length: i32) {
        self.position.x = match self.position.x + self.velocity.x {
            n if n < 0 => length + n,
            n if n >= length => n - length,
            n => n,
        };
        self.position.y = match self.position.y + self.velocity.y {
            n if n < 0 => height + n,
            n if n >= height => n - height,
            n => n,
        };
    }

    pub fn quadrant(&self, height: i32, length: i32) -> Option<usize> {
        let h_middle: i32 = length / 2;
        let v_middle: i32 = height / 2;
        match (self.position.x, self.position.y) {
            (l, h) if (0..h_middle).contains(&l) && (0..v_middle).contains(&h) => Some(0),
            (l, h) if (h_middle + 1..length).contains(&l) && (0..v_middle).contains(&h) => Some(1),
            (l, h) if (0..h_middle).contains(&l) && (v_middle + 1 ..height).contains(&h) => Some(2),
            (l, h) if (h_middle + 1..length).contains(&l) && (v_middle + 1..height).contains(&h) => Some(3),
            (_, _) => None,
        }
    }
}

fn main() {
    let re = Regex::new(r"(?m)^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for (_, [p_x, p_y, v_x, v_y]) in re
        .captures_iter(include_str!("../test.txt"))
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
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| r.moving(103, 101));
    }
    let res: usize = robots
        .iter_mut()
        .fold(vec![0; 4], |mut data, r| {
            println!("{:?}", r);
            if let Some(v) = r.quadrant(103, 101) {
                data[v] += 1
            };
            println!("{:?}", data);
            data
        })
        .iter()
        .product();
    println!("{res}");
}
