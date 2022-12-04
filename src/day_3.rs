use crate::utils::read_file;
use std::collections::HashSet;

pub fn sort_rucksacks() -> (i32, i32) {
    let input = read_file("input/3.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut count = 0;
    for line in lines.iter() {
        let compartment_size = line.len() / 2;

        let mut first_half: HashSet<&str> = HashSet::new();
        for i in 0..compartment_size {
            first_half.insert(&line[i..i + 1]);
        }

        let mut second_half: HashSet<&str> = HashSet::new();
        for i in compartment_size..line.len() {
            second_half.insert(&line[i..i + 1]);
        }

        // Find the intersection of the two sets
        let intersection = first_half.intersection(&second_half);
        let intersection_vec = Vec::from_iter(intersection);
        for item in intersection_vec.iter() {
            count += get_char(item)
        }
    }

    let mut count2 = 0;
    let mut badges = Vec::new();
    let mut group_count = 0;
    let mut group = String::new();
    for line in lines.iter() {
        // Find the character in each group of three lines
        group_count += 1;
        // Sort the string alphabetically
        let mut chars: Vec<char> = line.chars().collect();
        chars.sort();
        chars.dedup();
        let sorted_line = chars.into_iter().collect::<String>();
        group.push_str(&sorted_line);

        if group_count == 3 {
            for i in group.chars() {
                if group.matches(i).count() == 3 {
                    badges.push(i);
                    break;
                }
            }

            group_count = 0;
            group = String::new();
        }
    }

    for i in badges.iter() {
        count2 += get_char(&i.to_string());
    }

    (count as i32, count2 as i32)
}

fn get_char(character: &str) -> usize {
    let char = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    char.find(character).unwrap() + 1 // Indexes from zero but a needs to be 1
}
