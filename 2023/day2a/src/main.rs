fn check_game(game: &str) -> i32 {
    let RED = 12;
    let GREEN = 13;
    let BLUE = 14;

    let (id, data) = game.split_once(":").unwrap();

    println!("{} {}", id, data);

    0
}

fn main() {
    let mut sum: i32 = 0;

    let _ = include_str!("../input.txt")
        .lines();

    println!("{}", 0.1 + 0.2)
}
