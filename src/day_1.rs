use crate::utils;

pub fn get_calories() -> Vec<i32> {
    let input = utils::read_file("input/1.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut elves: Vec<i32> = Vec::new();
    let mut calories = 0;
    for line in lines.iter() {
        if line.len() == 0 {
            elves.push(calories);
            calories = 0;
        } else {
            let food_str = line;
            let food = food_str.parse::<i32>();
            calories += food.unwrap();
        }
    }

    // Now sort the elves by calories
    let mut sorted_elves = elves.clone();
    sorted_elves.sort();
    sorted_elves.reverse();

    let mut elves_to_return: Vec<i32> = sorted_elves.iter().take(3).cloned().collect();
    elves_to_return.push(elves_to_return.iter().sum());
    return elves_to_return;
}
