#[cfg(test)]
mod tests {
    use crate::get_elf_calories;

    #[test]
    fn part_one() {
        let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let output = get_elf_calories(&test_input);
        assert_eq!(output[0], 24000)
    }

    #[test]
    fn part_two() {
        let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let output: Vec<i32> = get_elf_calories(&test_input);
        println!("{:?}", output);
        assert_eq!(output.iter().sum::<i32>(), 45000)
    }
}
