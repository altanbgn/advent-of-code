fn main() {
    let mut sum = 0;

    let _ = include_str!("../input.txt")
        .lines()
        .for_each(|rucksack| {
            let mut duplicate_char = None;
            let rucksack_split = rucksack.split_at(rucksack.len() / 2);

            rucksack_split.0.chars().for_each(|char_1| {
                rucksack_split.1.chars().for_each(|char_2| {
                    if char_1 == char_2 {
                        duplicate_char = Some(char_1);
                    }
                });
            });

            let _ = match duplicate_char {
                Some(_) => {
                    let duplicate_char_as_num = duplicate_char.unwrap() as u32;

                    if duplicate_char_as_num >= 64 && duplicate_char_as_num < 97 {
                        return sum += duplicate_char.unwrap() as u32 - 38;
                    }
                    if duplicate_char_as_num >= 97 {
                        return sum += duplicate_char.unwrap() as u32 - 96;
                    }
                },
                None => ()
            };

        });

    println!("{}", sum);
}
