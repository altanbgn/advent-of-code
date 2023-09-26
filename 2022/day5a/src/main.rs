#[derive(Debug)]
struct Command {
    quantity: u32,
    from: u32,
    to: u32
}

fn cargo_parser(cargo_string: &str) -> Vec<Vec<char>> {
    let cargo_length = (cargo_string.chars().into_iter().position(|x| x as u32 == 10).unwrap() / 4) + 1;
    let mut cargo: Vec<Vec<char>> = vec![vec![]; cargo_length];

    let mut counter = 0;
    let mut row_index = 0;

    cargo_string.chars().into_iter().for_each(|item| {
        if item as u32 == 10 {
            row_index = 0;
            counter = 0;
            return
        }

        if counter == 1 {
            if item.is_ascii_digit() {
                counter += 1;
                return
            }
            if item.is_ascii_uppercase() {
                cargo[row_index].push(item);
                counter += 1;
                return
            }
        }

        if counter == 3 {
            row_index += 1;
            counter = 0;
            return
        }

        if item == ' ' || item == '[' || item == ']' {
            counter += 1;
            return
        }
    });

    let reversed_cargo = cargo.iter().map(|stack| {
        let mut cloned = stack.clone();
        cloned.reverse();

        cloned
    }).collect::<Vec<Vec<char>>>();

    reversed_cargo
}

fn part_one (command: &Command, parsed_cargo: &mut Vec<Vec<char>>) {
    for _ in 0..command.quantity {
        let popped_element = parsed_cargo[command.from as usize - 1].pop().unwrap();
        parsed_cargo[command.to as usize - 1].push(popped_element);
    }
}

fn part_two (command: &Command, parsed_cargo: &mut Vec<Vec<char>>) {
    let mut crate_memory: Vec<char> = vec![];

    for _ in 0..command.quantity {
        let popped_element = parsed_cargo[command.from as usize - 1].pop().unwrap();
        crate_memory.push(popped_element);
    }

    crate_memory.reverse();

    parsed_cargo[command.to as usize - 1].append(&mut crate_memory);
}

fn main() {
    let split_input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();

    let mut parsed_cargo = cargo_parser(split_input[0]);
    let cmd_list: Vec<&str> = split_input[1].lines().collect();

    cmd_list.iter().for_each(|cmd| {
        let parts: Vec<&str> = cmd.trim().split(" ").collect();

        let command = match parts[..] {
            ["move", count, "from", from, "to", to] => {
                Ok(Command {
                    quantity: count.parse().unwrap(),
                    from: from.parse().unwrap(),
                    to: to.parse().unwrap()
                })
            },
            _ => Err(()),
        }.unwrap();

        // part_one(&command, &mut parsed_cargo);
        // part_two(&command, &mut parsed_cargo);
    });

    parsed_cargo.iter().for_each(|stack| {
        println!("{:?}", stack[stack.len() - 1]);
    });
}
