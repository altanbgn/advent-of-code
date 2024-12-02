fn main() {
    let mut safe_lines = -1;

    include_str!("../input.txt").split("\n").for_each(|x| {
        let mut last_number: i32 = 0;
        let mut ascending: Option<bool> = None;
        let mut safe = true;

        for y in x.split_whitespace() {
            let current: i32 = y.parse().unwrap();

            if last_number == 0 {
                last_number = current;
                continue;
            }

            if ascending == None {
                if last_number > current {
                    ascending = Some(false);
                } else {
                    ascending = Some(true);
                }
            }

            if last_number == current {
                safe = false;
                break;
            }

            if (ascending == Some(false) && last_number < current)
                || (ascending == Some(true) && last_number > current)
            {
                safe = false;
                break;
            }

            let diff = (last_number - current).abs();
            if diff > 3 || diff == 0 {
                safe = false;
                break;
            }

            last_number = current;
        }

        if safe == true {
            safe_lines += 1;
        }
    });

    println!("{:?}", safe_lines);
}
