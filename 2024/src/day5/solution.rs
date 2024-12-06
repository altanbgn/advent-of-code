use crate::utils::print_answer;
use std::{cmp::Ordering, collections::HashMap};

pub fn section_a() {
    let mut sum = 0;

    include_str!("./input.txt")
        .split_once("\n\n")
        .into_iter()
        .for_each(|x| {
            let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

            x.0.split_whitespace().into_iter().for_each(|y| {
                let i = y.split("|").collect::<Vec<_>>();
                let key = i[0].parse::<i32>().unwrap();
                let value = i[1].parse::<i32>().unwrap();

                rules_map
                    .entry(key)
                    .and_modify(move |e| e.push(value))
                    .or_insert(vec![value]);
            });

            x.1.split_whitespace().into_iter().for_each(|y| {
                let mut vec = y
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                println!("{:?}: vec", vec);

                vec.sort_by(|x, y| {
                    if rules_map.contains_key(y) {
                        let mut exists = false;

                        rules_map
                            .get(y)
                            .unwrap()
                            .into_iter()
                            .for_each(|h| {
                                if h == x {
                                    exists = true
                                }
                            });

                        if exists {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                    }

                    Ordering::Less
                });

                println!("{:?}: sorted vec", vec);
                println!("{:?}", vec[(vec.len() as f32 / 2.0).floor() as usize]);

                sum += vec[(vec.len() as f32 / 2.0).floor() as usize];
            });
        });

    print_answer(5, "A", sum);
}

pub fn section_b() {
    print_answer(5, "B", -1);
}
