use crate::utils::print_answer;
use regex::Regex;

pub fn section_a() {
    let pattern: Regex = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let input: String = include_str!("./input.txt").to_string();
    let mut sum: i32 = 0;

    pattern.captures_iter(&input).for_each(|x| {
        let first_number: i32 = x.name("first").unwrap().as_str().parse().unwrap();
        let second_number: i32 = x.name("second").unwrap().as_str().parse().unwrap();
        sum += first_number * second_number;
    });

    print_answer(3, "A", sum);
}

pub fn section_b() {
    let pattern: Regex = Regex::new(r"((?<actionDont>don't\(\))|(?<actionDo>do\(\))|mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\))").unwrap();
    let input: String = include_str!("./input.txt").to_string();
    let mut disabled: bool = false;
    let mut sum: i32 = 0;

    pattern.captures_iter(&input).for_each(|x| {
        if x.name("actionDo").is_some() {
            disabled = false;
        }
        if x.name("actionDont").is_some() {
            disabled = true;
        }
        if x.name("first").is_some() && x.name("second").is_some() && disabled == false {
            let first_number: i32 = x.name("first").unwrap().as_str().parse().unwrap();
            let second_number: i32 = x.name("second").unwrap().as_str().parse().unwrap();
            sum += first_number * second_number;
        }
    });

    print_answer(3, "B", sum);
}

