mod lib;
use std::fs;

fn main() {
    let input = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/day1/input"))
        .expect("Something went wrong reading the file");
    let elf_calories = get_elf_calories(&input);
    println!("The elf carrying the most calories has {}, the second has {}, the third has {}, and the sum of those top three is {}", elf_calories[0], elf_calories[1], elf_calories[2], elf_calories.iter().sum::<i32>());
}

fn get_elf_calories(input: &str) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut elves: Vec<i32> = Vec::new();
    let mut calories = 0;
    for line in lines.iter() {
        if line.is_empty() {
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

    let elves_to_return: Vec<i32> = sorted_elves.iter().take(3).cloned().collect();
    elves_to_return
}

#[cfg(test)]
mod tests {
    use crate::get_elf_calories;

    const TEST_INPUT: &str = " 1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_one() {
        let output = get_elf_calories(&TEST_INPUT);
        assert_eq!(output[0], 24000)
    }

    #[test]
    fn part_two() {
        let output: Vec<i32> = get_elf_calories(&TEST_INPUT);
        println!("{:?}", output);
        assert_eq!(output.iter().sum::<i32>(), 45000)
    }
}
