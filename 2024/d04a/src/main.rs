fn test_word(word: [char; 4], index_data: usize, data: &[char], range: usize) -> i32 {
    for (index_word, letter) in word.into_iter().enumerate() {
        let offset = index_data + index_word * range;
        if data.get(offset) != Some(&letter) {
            return 0;
        }
    }
    1
}

fn main() {
    let data = include_str!("../input.txt").chars().collect::<Vec<char>>();
    let mut res = 0;
    let row_size = data.iter().position(|n| *n == '\n').unwrap() + 1;
    for (index_data, data_point) in data.iter().enumerate() {
        if *data_point == 'X' {
            for range in [1, row_size - 1, row_size, row_size + 1] {
                res += test_word(['X', 'M', 'A', 'S'], index_data, &data, range);
            }
        } else if *data_point == 'S' {
            for range in [1, row_size - 1, row_size, row_size + 1] {
                res += test_word(['S', 'A', 'M', 'X'], index_data, &data, range);
            }
        }
    }
    println!("{res:?}");
}
