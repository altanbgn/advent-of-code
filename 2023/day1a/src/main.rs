fn main() {
    let mut sum = 0;
    let _ = include_str!("../input.txt")
        .split("\n")
        .for_each(|x| {
            let mut first: Option<i32> = None;
            let mut last: Option<i32> = None;

            x.chars().for_each(|y| {
                if y.is_digit(10) {
                    if first.is_none() {
                        first = Some(y.to_digit(10).unwrap() as i32);
                    }

                    last = Some(y.to_digit(10).unwrap() as i32);
                }
            });

            sum += (first.unwrap_or(0) * 10) + last.unwrap_or(0);
        });
}
