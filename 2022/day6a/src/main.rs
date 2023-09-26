fn main() {
    let buffer: Vec<char> = include_str!("../input.txt").trim().chars().collect();
    let mut num: usize = 0;

    buffer.iter().enumerate().for_each(|(index, character)| {
        if num != 0 {
            return;
        }

        if index + 3 >= buffer.len() {
            return;
        }

        if character != &buffer[index + 1]
            && character != &buffer[index + 2]
            && character != &buffer[index + 3]
            && &buffer[index + 1] != &buffer[index + 2]
            && &buffer[index + 1] != &buffer[index + 3]
            && &buffer[index + 2] != &buffer[index + 3]
        {
            println!("{}, {}, {}, {}", character, &buffer[index + 1], &buffer[index + 2], &buffer[index + 3]);
            num = index + 4;
            return;
        }
    });

    println!("{}", num);
}
