fn main() {
    let mut left_array: Vec<i32> = vec![];
    let mut right_array: Vec<i32> = vec![];

    // Parsing from input.txt
    include_str!("../input.txt").split("\n").for_each(|x| {
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

    println!("SUM: {:?}", sum);
}
