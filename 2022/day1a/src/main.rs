fn main() {
    let calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|food_calorie| food_calorie.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .max()
        .unwrap();

    println!("{}", calories)
}
