use crate::utils::print_answer;

pub fn section_a() {
    // TODO: try other method
    let mut word = [0; 4];
    let input_matrix: Vec<Vec<char>> = include_str!("./input.txt")
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let result = (0..input_matrix[0].len() as isize)
        .flat_map(|x| (0..input_matrix.len() as isize).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // Top right
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // Right
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // Bottom right
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // Bottom
            ]
        })
        .filter(|coordinates| {
            let mut fetched_chars = coordinates.iter().map(|(x, y)| {
                input_matrix
                    .get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });

            word.fill_with(|| fetched_chars.next().unwrap_or_default() as u8);
            &word == b"XMAS" || &word == b"SAMX"
        })
        .count();

    print_answer(4, "A", result);
}

pub fn section_b() {
    // TODO: try other method
    let mut x = [0; 4];
    let input_matrix: Vec<Vec<char>> = include_str!("./input.txt")
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let result = (0..input_matrix[0].len() as isize)
        .flat_map(|x| (0..input_matrix.len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                (x + 1, y + 1), // Center
                (x, y),         // Top right
                (x, y + 2),     // Bottom right
                (x + 2, y),     // Top left
                (x + 2, y + 2), // Bottom left
            ]
        })
        .filter(|coordinates| {
            let mut fetched_chars = coordinates.iter().map(|(x, y)| {
                input_matrix
                    .get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });

            if fetched_chars.next().is_none_or(|n| n != 'A') {
                return false;
            }

            x.fill_with(|| fetched_chars.next().unwrap_or_default() as u8);
            &x == b"MMSS" || &x == b"MSMS" || &x == b"SSMM" || &x == b"SMSM"
        })
        .count();

    print_answer(4, "B", result);
}

