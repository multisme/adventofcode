use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

fn norm(a: &Junction, b: &Junction) -> i64 {
    (b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)
}

fn parse_data(data: &'static str) -> (Vec<Junction>, usize) {
    (
        data.lines()
            .map(|l| {
                let mut coord = l.split(',');
                Junction {
                    x: coord.next().map(|n| n.parse::<i64>().unwrap()).unwrap(),
                    y: coord.next().map(|n| n.parse::<i64>().unwrap()).unwrap(),
                    z: coord.next().map(|n| n.parse::<i64>().unwrap()).unwrap(),
                }
            })
            .collect(),
        data.lines().count(),
    )
}

fn link_smaller_junction(junctions: Vec<Junction>) -> HashMap<(Junction, Junction), i64> {
    let mut links_map: HashMap<(Junction, Junction), i64> = HashMap::new();
    junctions.iter().for_each(|j| {
        junctions.iter().for_each(|i| {
            if i < j {
                links_map.entry((*i, *j)).or_insert(norm(i, j));
            } else if i > j {
                links_map.entry((*j, *i)).or_insert(norm(i, j));
            }
        });
    });
    links_map
}

fn sort_circuits(links_map: &mut HashMap<(Junction, Junction), i64>) -> Vec<BTreeSet<Junction>> {
    let mut sorted_links_map = links_map.iter().collect::<Vec<_>>();
    sorted_links_map.sort_by(|a, b| a.1.cmp(b.1));
    sorted_links_map
        .iter()
        .map(|((a, b), _distance)| {
            let mut set = BTreeSet::new();
            set.insert(*a);
            set.insert(*b);
            set
        })
        .collect::<Vec<BTreeSet<Junction>>>()
}

fn link_circuits(
    circuits: Vec<BTreeSet<Junction>>,
    mut circuit: BTreeSet<Junction>,
) -> Vec<BTreeSet<Junction>> {
    let mut linked_circuits = Vec::new();
    for cur_circuit in circuits {
        if !cur_circuit.is_disjoint(&circuit) {
            circuit = cur_circuit
                .union(&circuit)
                .cloned()
                .collect::<BTreeSet<Junction>>();
            continue;
        }
        linked_circuits.push(cur_circuit.clone());
    }
    // If there is not union possible we push the new circuit in the circuit list;
    linked_circuits.push(circuit.clone());
    linked_circuits
}

fn find_circuits(circuits: Vec<BTreeSet<Junction>>, limit: usize) -> Vec<BTreeSet<Junction>> {
    let mut found_circuits: Vec<BTreeSet<Junction>> = Vec::new();
    let mut last_circuit = Vec::new();
    for circuit in circuits.into_iter() {
        found_circuits = link_circuits(found_circuits, circuit.clone());
        if found_circuits.len() == 1 && found_circuits.first().unwrap().clone().len() == limit {
            last_circuit.push(circuit.clone());
            return last_circuit;
        }
    }
    found_circuits
}

fn compute_result(circuits: Vec<BTreeSet<Junction>>) -> usize {
    let last_circuit = circuits.first().unwrap();
    last_circuit.iter().fold(1, |acc, e| acc * e.x as usize)
}

#[allow(dead_code)]
fn print_circuits(circuits: Vec<BTreeSet<Junction>>) {
    println!("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm");
    for circuit in circuits {
        println!("{circuit:?}");
    }
    println!("ooooooooooooooooooooooooooooooooooo");
}

fn main() {
    let (junctions, limit) = parse_data(include_str!("../input.txt"));
    let sets_size = compute_result(find_circuits(
        sort_circuits(&mut link_smaller_junction(junctions)),
        limit,
    ));
    println!("{sets_size:?}");
    //println!("{}", sets_size.iter().take(3).product::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let (map, limit) = parse_data(input);
        let res = compute_result(find_circuits(
            sort_circuits(&mut link_smaller_junction(map)),
            limit,
        ));
        assert_eq!(25272, res);
    }
}
