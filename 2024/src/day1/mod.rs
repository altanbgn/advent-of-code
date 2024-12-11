use crate::utils::print_answer;

pub fn section_a() {
    let mut left_array: Vec<i32> = vec![];
    let mut right_array: Vec<i32> = vec![];

    // Parsing from input.txt
    include_str!("./input.txt").split("\n").for_each(|x| {
        let line: Vec<&str> = x.split("   ").collect();
        match line[0].parse() {
            Ok(first_number) => left_array.push(first_number),
            Err(..) => return,
        };

        match line[1].parse() {
            Ok(second_number) => right_array.push(second_number),
            Err(..) => return,
        };
    });

    // Sort them
    left_array.sort();
    right_array.sort();

    let mut sum = 0;
    // Sum of difference's absolute value
    for index in 0..left_array.len() {
        sum += (left_array[index] - right_array[index]).abs();
    }

    print_answer(1, "A", sum);
}

pub fn section_b() {
    let mut numbers = std::collections::HashMap::<i32, i32>::new();
    let mut second_column: Vec<i32> = vec![];

    // Parsing from input.txt
    include_str!("./input.txt").split("\n").for_each(|x| {
        let line: Vec<&str> = x.split_whitespace().collect();

        if line.len() > 0 {
            match line[0].parse() {
                Ok(first_number) => numbers.insert(first_number, 0),
                Err(..) => return,
            };

            match line[1].parse() {
                Ok(second_number) => second_column.push(second_number),
                Err(..) => return,
            };
        }
    });

    // Iterate through second column and count the numbers
    second_column.into_iter().for_each(|x| {
        if numbers.contains_key(&x) {
            numbers.insert(x, numbers.get(&x).unwrap() + 1);
        };
    });

    // Sum it up
    let mut sum = 0;
    numbers.into_iter().for_each(|x| {
        sum += x.0 * x.1;
    });

    print_answer(1, "B", sum);
}

