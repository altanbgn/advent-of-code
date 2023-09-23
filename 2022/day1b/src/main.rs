fn main() {
    let mut calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|food_calorie| food_calorie.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .collect::<Vec<i32>>();

    calories.sort();

    println!("{}", calories.into_iter().rev().take(3).sum::<i32>());
}
