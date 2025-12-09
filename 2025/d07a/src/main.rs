use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position{
    x: usize,
    y: usize
}

struct Limits{
    length: usize,
    height: usize,
}


impl Position{
    fn left(&self, ends: &Limits) -> Option<Position> {
        if (1..ends.length).contains(&self.x){
            Some(Position { x: self.x - 1, y: self.y })
        } else {
            None
        }
    }
    
    fn right(&self, ends: &Limits) -> Option<Position> {
        if (1..ends.length).contains(&self.x){
            Some(Position { x: self.x + 1, y: self.y })
        } else {
            None
        }
    }
}


fn parse_data(data: &'static str) -> (Vec<Position>, VecDeque<Position>, Limits) {
    let mut splitters = vec![];
    let mut starts = VecDeque::new();
    let (mut length, mut height) = (0, 0);
    for (y, l) in data.lines().enumerate() {
       for (x, c) in l.chars().enumerate() {
            if c == '^'{
                splitters.push(Position{x, y})
            } else if c == 'S'{
                starts.push_back(Position{x, y})
            }
           length = x;
       }
        height = y;
    }
    (splitters, starts, Limits{length, height})
}

fn beaming(splitters: Vec<Position>, starts: &mut VecDeque<Position>, ends: Limits) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    while let Some(start) = starts.pop_back(){
           let mut candidates: Vec<&Position> = splitters.iter().filter(|s| s.x == start.x && s.y > start.y).collect();
            candidates.sort_by(|a,b| a.y.cmp(&b.y));
            if let Some(hit) = candidates.first() && !visited.contains(*hit){
                visited.insert(**hit);
                if let Some(beam) = hit.left(&ends){
                        starts.push_front(beam);
                }
                if let Some(beam) = hit.right(&ends){
                        starts.push_front(beam);
                }
            }
        }
    visited.len()
}

fn main() {
    let (splitters, mut starts, ends) = parse_data(include_str!("../input.txt"));
    println!("{}", beaming(splitters, &mut starts, ends));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

        let (splitters, mut starts, ends) = parse_data(input);
        assert_eq!(21, beaming(splitters, &mut starts, ends));
    }
}
