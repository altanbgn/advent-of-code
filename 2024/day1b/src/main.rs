fn main() {
    let mut numbers = std::collections::HashMap::<i32, i32>::new();
    let mut second_column: Vec<i32> = vec![];

    // Parsing from input.txt
    include_str!("../input.txt").split("\n").for_each(|x| {
        let line: Vec<&str> = x.split("   ").collect();
        match line[0].parse() {
            Ok(first_number) => numbers.insert(first_number, 0),
            Err(..) => return,
        };

        match line[1].parse() {
            Ok(second_number) => second_column.push(second_number),
            Err(..) => return,
        };
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

    println!("SUM: {:?}", sum);
}
