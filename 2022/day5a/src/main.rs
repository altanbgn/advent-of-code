fn main() {
    let cargo: Vec<Vec<char>> = vec![vec![]];

    include_str!("../input.txt")
        .split("\n")
        .for_each(|item| {
            let pog: Vec<_> = item.splitn(1, "[ ]").collect();
            println!("{:?}", pog);
        })
}
