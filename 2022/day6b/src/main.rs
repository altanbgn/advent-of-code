fn main() {
    let buffer: Vec<char> = include_str!("../input.txt").trim().chars().collect();
    let mut num: usize = 0;

    buffer.iter().enumerate().for_each(|(index, _character)| {
        if num != 0 {
            return;
        }

        if index + 3 >= buffer.len() {
            return;
        }

        let mut passed: bool = true;

        for i in 0..14 {
            for j in i..14 {
                if &buffer[index + i] == &buffer[index + j] && i != j {
                    passed = false;
                }
            }
        }

        if passed == true {
            num = index + 14;
            return;
        }
    });

    println!("{}", num);
}
