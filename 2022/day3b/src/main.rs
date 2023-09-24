fn main() {
    let mut sum = 0;

    let mut count = 0;
    let mut first_rucksack: Option<String> = None;
    let mut second_rucksack: Option<String> = None;

    include_str!("../input.txt").lines().for_each(|item| {
        if count == 0 {
            count += 1;
            first_rucksack = Some(item.to_string());
            return;
        }
        if count == 1 {
            count += 1;
            second_rucksack = Some(item.to_string());
            return;
        }
        if count == 2 {
            // Comparing starts!
            let mut key: Option<char> = None;

            count = 0;

            first_rucksack.as_ref().unwrap().chars().for_each(|item_1| {
                second_rucksack
                    .as_ref()
                    .unwrap()
                    .chars()
                    .for_each(|item_2| {
                        item.chars().for_each(|item_3| {
                            if item_1 == item_2 && item_2 == item_3 {
                                key = Some(item_1);
                            }
                        })
                    });
            });

            match key {
                Some(_) => {
                    let key_as_num = key.unwrap() as u32;

                    if key_as_num >= 64 && key_as_num < 97 {
                        return sum += key.unwrap() as u32 - 38;
                    }
                    if key_as_num >= 97 {
                        return sum += key.unwrap() as u32 - 96;
                    }
                }
                None => (),
            }
        }
    });

    println!("{}", sum);
}
