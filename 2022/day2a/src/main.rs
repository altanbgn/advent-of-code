fn main() {
    let mut sum = 0;

    let _ = include_str!("../input.txt")
        .split("\n")
        .for_each(|item| {
            let _ = match item.chars().nth(2) {
                Some('X') => {
                    sum += 1;
                    match item.chars().nth(0) {
                        Some('A') => sum += 3,
                        Some('C') => sum += 6,
                        _ => (),
                    }
                }
                Some('Y') => {
                    sum += 2;
                    match item.chars().nth(0) {
                        Some('A') => sum += 6,
                        Some('B') => sum += 3,
                        _ => ()
                    }
                },
                Some('Z') => {
                    sum += 3;
                    match item.chars().nth(0) {
                        Some('B') => sum += 6,
                        Some('C') => sum += 3,
                        _ => ()
                    }
                },
                _ => ()
            };
        });

    println!("{}", sum);
}
