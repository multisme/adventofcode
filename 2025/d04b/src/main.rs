use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Tile {
    Roll,
    Empty,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_data(data: &'static str) -> HashMap<Position, Tile> {
    let mut grid = HashMap::new();
    data.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, letter)| {
            let tiletype = match letter {
                '.' => Tile::Empty,
                _ => Tile::Roll,
            };
            grid.insert(Position{x: x as i32, y: y as i32}, tiletype);
        })
    });
    grid
}

fn clean_accessible_rolls(grid: &mut HashMap<Position, Tile>) -> Option<usize>{
    let around = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];
    let mut to_clean = vec![];
    let mut count = 0;
    for (pos, tile) in grid.clone().iter_mut() {
        let mut n_rolls = 0;
        for n in around {
            if let Some( Tile::Roll ) = grid.get(&Position{x: pos.x + n.0, y: pos.y + n.1}) { n_rolls +=1 }
        }
        if n_rolls < 4 && *tile == Tile::Roll{
            to_clean.push(pos.clone());
            count += 1;
        }
    };
    for pos in to_clean.iter() {
        grid.insert(pos.clone(), Tile::Empty);
    }
    match count {
        0 => None,
        _=> Some(count)
    }
}

fn clean_warehouse(grid: &mut HashMap<Position, Tile>) -> usize{
        let mut total_removed = 0;
        while let Some(removed) =  clean_accessible_rolls(grid) {
            total_removed += removed
        }
    total_removed
}

fn main() {
    let mut input = parse_data(include_str!("../input.txt"));
    println!("{}", clean_warehouse(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut input = parse_data(
            ".@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@."
        );
        assert_eq!(43, clean_warehouse(&mut input))
    }
}
