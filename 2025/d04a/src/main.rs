use std::collections::HashMap;

#[derive(PartialEq)]
enum TileType {
    Roll,
    Empty,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct Tile {
    tiletype: TileType,
    value: u32,
}

fn parse_data(data: &'static str) -> HashMap<Position, Tile> {
    let mut grid = HashMap::new();
    data.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, letter)| {
            let value = 0;
            let tiletype = match letter {
                '.' => TileType::Empty,
                _ => TileType::Roll,
            };
            grid.insert(Position{x: x as i32, y: y as i32}, Tile{tiletype, value});
        })
    });
    grid
}

fn count_accessible_rolls(grid: HashMap<Position, Tile>) -> usize{
    let around = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];
    let mut count = 0;
    for (pos, tile ) in grid.iter() {
        let mut n_rolls = 0;
        for n in around {
            if let Some(Tile { tiletype: TileType::Roll , .. }) = grid.get(&Position{x: pos.x + n.0, y: pos.y + n.1}) { n_rolls +=1 }
        }
        if n_rolls < 4 && tile.tiletype == TileType::Roll{
            println!("{pos:?}");
            count += 1;
        }
    };
    count
}

fn main() {
    let input = parse_data(include_str!("../input.txt"));
    println!("{}", count_accessible_rolls(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(13, count_accessible_rolls(parse_data(input)))
    }
}
