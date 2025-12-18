use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    timelines: usize,
    processed: bool,
}

struct Limits {
    length: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn left(&self, ends: &Limits) -> Option<Position> {
        if (1..ends.length).contains(&self.x) {
            Some(Position {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn right(&self, ends: &Limits) -> Option<Position> {
        if (1..ends.length).contains(&self.x) {
            Some(Position {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }
}

fn parse_data(data: &'static str) -> (Vec<Position>, Position, Limits) {
    let mut splitters = Vec::new();
    let mut starts = Position { x: 0, y: 0 };
    let (mut length, mut height) = (0, 0);
    for (y, l) in data.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '^' {
                splitters.push(Position { x, y });
            } else if c == 'S' {
                splitters.push(Position { x, y });
                starts = Position { x, y };
            }
            length = x;
        }
        height = y;
    }
    (splitters, starts, Limits { length, height })
}

fn beaming(
    splitters: &Vec<Position>,
    cache: &mut HashMap<Position, Node>,
    start: Position,
    ends: &Limits,
) -> usize {
    let mut candidates: Vec<&Position> = splitters
        .iter()
        .filter(|s| s.x == start.x && s.y > start.y)
        .collect();
    candidates.sort_by(|a, b| a.y.cmp(&b.y));

    if let Some(hit) = candidates.first() {
        if let Some(start_node) = cache.get_mut(hit)
            && start_node.processed
        {
            return start_node.timelines;
        }
        let mut timelines = 0;
        if let Some(beam) = hit.left(ends) {
            timelines += beaming(splitters, cache, beam, ends);
        }
        if let Some(beam) = hit.right(ends) {
            timelines += beaming(splitters, cache, beam, ends);
        }
        cache.insert(
            **hit,
            Node {
                processed: true,
                timelines,
            },
        );
        timelines
    } else {
        1
    }
}

fn main() {
    let (splitters, starts, ends) = parse_data(include_str!("../input.txt"));
    println!(
        "{}",
        beaming(&splitters, &mut HashMap::new(), starts, &ends)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

        let (splitters, starts, ends) = parse_data(input);
        assert_eq!(40, beaming(&splitters, &mut HashMap::new(), starts, &ends));
    }
}
