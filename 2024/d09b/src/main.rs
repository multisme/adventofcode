
#[derive(PartialEq, Debug)]
enum MemoryType {
    Free,
    Freed,
    Full,
}

#[derive(PartialEq, Debug)]
struct MemorySegment {
    memory_type: MemoryType,
    size: usize,
    index: usize,
}

fn main() {
    let data = include_str!("../input.txt");

    let disk_map = data
        .chars()
        .take(data.len() - 1)
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut indice = true;
    let mut count = 0;
    let mut memory = disk_map
        .iter()
        .map(|v| {
            indice = !indice;
            if indice {
                count += 1;
                MemorySegment {
                    memory_type: MemoryType::Free,
                    size: *v,
                    index: 0,
                }
            } else {
                MemorySegment {
                    memory_type: MemoryType::Full,
                    size: *v,
                    index: count,
                }
            }
        })
        .collect::<Vec<MemorySegment>>();
    for index in 0..memory.len() - 1 {
        let tail = memory.len() - index - 1;
        if memory[tail].memory_type != MemoryType::Full {
            continue;
        }
        for head in 0..tail {
            if memory[head].memory_type != MemoryType::Free {
                continue;
            }
            if memory[head].size > memory[tail].size {
                let new_free_segement = MemorySegment {
                    memory_type: MemoryType::Free,
                    size: memory[head].size - memory[tail].size,
                    index: 0,
                };
                memory[head] = MemorySegment {
                    memory_type: MemoryType::Full,
                    size: memory[tail].size,
                    index: memory[tail].index,
                };
                memory[tail] = MemorySegment {
                    memory_type: MemoryType::Freed,
                    index: 0,
                    ..memory[tail]
                };
                memory.insert(head + 1, new_free_segement);
                break;
            } else if memory[head].size == memory[tail].size {
                memory[head] = MemorySegment {
                    memory_type: MemoryType::Full,
                    size: memory[tail].size,
                    index: memory[tail].index,
                };
                memory[tail] = MemorySegment {
                    memory_type: MemoryType::Freed,
                    index: 0,
                    ..memory[tail]
                };
                break;
            }
        }
    }
    let result = memory
        .iter()
        .flat_map(|s| vec![s.index; s.size])
        .enumerate()
        .fold(0, |acc, (index, size)| acc + size * index);
    println!("{result:?}");
}
