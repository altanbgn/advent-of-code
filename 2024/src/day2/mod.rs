use crate::utils::print_answer;

pub fn section_a() {
    let mut safe_lines = -2;

    include_str!("./input.txt").split("\n").for_each(|x| {
        let list: Vec<&str> = x.split_whitespace().collect();

        if check_safe(list.clone()) == true {
            safe_lines += 1;
        }
    });

    print_answer(2, "A", safe_lines);
}

pub fn section_b() {
    let mut safe_lines = -2;

    include_str!("./input.txt").split("\n").for_each(|x| {
        let list: Vec<&str> = x.split_whitespace().collect();

        if check_safe(list.clone()) == true {
            safe_lines += 1;
        } else {
            for n in 0..list.len() {
                let mut new_list = list.clone();
                new_list.remove(n);
                if check_safe(new_list.clone()) == true {
                    safe_lines += 1;
                    break;
                }
            }
        };
    });

    print_answer(2, "B", safe_lines);
}

fn check_safe(list: Vec<&str>) -> bool {
    let mut last_number: i32 = 0;
    let mut ascending: Option<bool> = None;
    let mut safe = true;

    for x in list.into_iter() {
        let current: i32 = x.parse().unwrap();

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

    return safe;
}

