fn main() {
    let mut sum = 0;
    let _ = include_str!("../input.txt")
        .split("\n")
        .for_each(|x| {
            let replaced_str = x.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3hree")
                .replace("four", "f4our")
                .replace("five", "f5ive")
                .replace("six", "s6x")
                .replace("seven", "s7even")
                .replace("eight", "e8ight")
                .replace("nine", "n9ne");

            let mut first: Option<i32> = None;
            let mut last: Option<i32> = None;

            replaced_str.chars().for_each(|y| {
                if y.is_digit(10) {
                    if first.is_none() {
                        first = Some(y.to_digit(10).unwrap() as i32);
                    }

                    last = Some(y.to_digit(10).unwrap() as i32);
                }
            });

            sum += (first.unwrap_or(0) * 10) + last.unwrap_or(0);
        });

    println!("Sum: {}", sum)
}
