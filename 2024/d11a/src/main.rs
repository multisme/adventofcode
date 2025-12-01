
#[derive(Clone, Debug)]
enum Stone {
    Str(String),
    Num(u64),
}

impl Stone {
    fn convert(&mut self) -> Self {
        match self {
            Stone::Str(value) => Stone::Num(value.parse::<u64>().unwrap()),
            Stone::Num(value) => Stone::Str(value.to_string()),
        }
    }

    fn len(&mut self) -> usize {
        match self {
            Stone::Str(value) => value.len(),
            Stone::Num(0) => 1,
            Stone::Num(value) => (value.ilog10() + 1) as usize,
        }
    }

    fn split(&self, len: usize) -> [Stone; 2] {
        match self {
            Stone::Num(value) => {
                let div = 10_u64.pow(len as u32 / 2);
                [Stone::Num(*value / div), Stone::Num(*value % div)]
            }
            Stone::Str(value) => {
                let (left, right) = value.split_at(len / 2);
                [Stone::Str(left.to_string()), Stone::Str(right.to_string())]
            }
        }
    }
    fn change(&self) -> Vec<Stone> {
        match self {
            Stone::Num(value) => {
                let size = Stone::Num(*value).len();
                if size % 2 == 0 {
                    self.split(size).to_vec()
                } else if *value == 0 {
                    [Stone::Num(1)].to_vec()
                } else {
                    [Stone::Num(value * 2024)].to_vec()
                }
            }
            _ => [].to_vec(),
        }
    }
}

fn main() {
    let data = include_str!("../input.txt")
        .split_whitespace()
        .map(|v| Stone::Num(v.parse::<u64>().unwrap()))
        .collect::<Vec<Stone>>();
    let mut stones = data;
    for _i in 0..25 {
        stones = stones.iter().flat_map(|x| x.change()).collect();
    }
    println!("{}", stones.len());
}
