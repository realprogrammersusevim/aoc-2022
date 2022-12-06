#![allow(dead_code)]
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod utils;

fn main() {
    // Day 1
    println!("Day 1");
    println!("=======");
    let elf_calories = day_1::get_calories();
    println!("The elf carrying the most calories has {}, the second has {}, the third has {}, and the sum of those top three is {}", elf_calories[0], elf_calories[1], elf_calories[2], elf_calories[3]);

    // Day 2
    println!("Day 2");
    println!("=======");
    println!("Total points: {}", day_2::eval_strat(true)); // For puzzle one
    println!("Total points: {}", day_2::eval_strat(false)); // For puzzle two
    println!("");

    // Day 3
    println!("Day 3");
    println!("=======");
    println!("Total points: {:?}", day_3::sort_rucksacks().0,);
    println!("Points of badges: {:?}", day_3::sort_rucksacks().1);
    println!("");

    // Day 4
    println!("Day 4");
    println!("=======");
    let results = day_4::get_overlap();
    println!("Number completely contained: {}", results.0);
    println!("Number overlapping: {}", results.1);
    println!("");

    // Day 5
    println!("Day 5");
    println!("=======");
    println!("{:?}", day_5::sim_stack());
    println!("");

    // Day 6
    println!("Day 6");
    println!("=======");
    println!("Message start index: {}", day_6::find_begin(4));
    println!("Message end index: {}", day_6::find_begin(14));
}
