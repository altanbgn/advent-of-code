fn main() {
    let mut sum = 0;

    include_str!("../input.txt").lines().for_each(|item| {
        let pairs: Vec<&str> = item.split(",").collect();

        let first_section: Vec<i32> = pairs[0]
            .split("-")
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        let second_section: Vec<i32> = pairs[1]
            .split("-")
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        if (first_section[0] >= second_section[0] && first_section[0] <= second_section[1])
        || (first_section[1] >= second_section[0] && first_section[1] <= second_section[1])
        || (second_section[0] >= first_section[0] && second_section[0] <= first_section[1])
        || (second_section[1] >= first_section[0] && second_section[1] <= first_section[1])
        {
            sum += 1;
        }
    });

    println!("{}", sum);
}
