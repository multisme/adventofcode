fn test_word(word: [char; 3], index_data: usize, data: &[char], range: usize) -> bool {
    for (index_word, letter) in word.iter().enumerate() {
        let offset = index_data + index_word * range;
        if data.get(offset) != Some(letter) {
            return false;
        }
    }
    true
}

fn main() {
    let data = include_str!("../input.txt").chars().collect::<Vec<char>>();
    let mut res = 0;
    let row_size = data.iter().position(|x| *x == '\n').unwrap() + 1;
    for (index_data, data_point) in data.iter().enumerate() {
        if *data_point == 'M'
            && test_word(['M', 'A', 'S'], index_data, &data, row_size + 1)
            && test_word(['S', 'A', 'M'], index_data + 2, &data, row_size - 1)
            || test_word(['M', 'A', 'S'], index_data, &data, row_size + 1)
                && test_word(['M', 'A', 'S'], index_data + 2, &data, row_size - 1)
        {
            res += 1;
        } else if *data_point == 'S'
            && test_word(['S', 'A', 'M'], index_data, &data, row_size + 1)
            && test_word(['M', 'A', 'S'], index_data + 2, &data, row_size - 1)
            || test_word(['S', 'A', 'M'], index_data, &data, row_size + 1)
                && test_word(['S', 'A', 'M'], index_data + 2, &data, row_size - 1)
        {
            res += 1
        }
    }
    println!("{res:?}");
}
