mod day_1;
mod utils;

fn main() {
    let elf_calories = day_1::get_calories();
    println!("The elf carrying the most calories has {}, the second has {}, the third has {}, and the sum of those top three is {}", elf_calories[0], elf_calories[1], elf_calories[2], elf_calories[3])
}
