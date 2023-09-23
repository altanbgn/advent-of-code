fn main() {
    let mut sum = 0;

    let _ = include_str!("../input.txt")
        .split("\n")
        .for_each(|item| {
            let _ = match item.chars().nth(2) {
                Some('X') => {
                    sum += 0;
                    match item.chars().nth(0) {
                        Some('A') => sum += 3,
                        Some('B') => sum += 1,
                        Some('C') => sum += 2,
                        _ => (),
                    }
                }
                Some('Y') => {
                    sum += 3;
                    match item.chars().nth(0) {
                        Some('A') => sum += 1,
                        Some('B') => sum += 2,
                        Some('C') => sum += 3,
                        _ => ()
                    }
                },
                Some('Z') => {
                    sum += 6;
                    match item.chars().nth(0) {
                        Some('A') => sum += 2,
                        Some('B') => sum += 3,
                        Some('C') => sum += 1,
                        _ => ()
                    }
                },
                _ => ()
            };
        });

    println!("{}", sum);
}
