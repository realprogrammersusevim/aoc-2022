use crate::utils::read_file;
use regex::Regex;

// I've loved functions, but I think it's time I used a struct
struct Elf {
    job_start: i32,
    job_end: i32,
}

pub fn get_overlap() -> (i32, i32) {
    let input = read_file("input/4.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut contained = 0;
    let mut overlapping = 0;
    // Needs to match these patterns 30-4, 1-99
    let regex = Regex::new(r"(\d{1,2})-(\d{1,2}),(\d{1,2})-(\d{1,2})").unwrap();
    for line in lines.iter() {
        // Capture indexing starts with the leftmost capture beginning at 1 (0 is the whole match)
        let cap = regex.captures(line);
        match cap {
            Some(c) => {
                let elf1 = Elf {
                    job_start: c[1].parse::<i32>().unwrap(),
                    job_end: c[2].parse::<i32>().unwrap(),
                };
                let elf2 = Elf {
                    job_start: c[3].parse::<i32>().unwrap(),
                    job_end: c[4].parse::<i32>().unwrap(),
                };
                if elf1.job_start <= elf2.job_start && elf1.job_end >= elf2.job_end {
                    contained += 1;
                    overlapping += 1;
                } else if elf2.job_start <= elf1.job_start && elf2.job_end >= elf1.job_end {
                    contained += 1;
                    overlapping += 1;
                } else if elf1.job_start <= elf2.job_start && elf1.job_end >= elf2.job_start {
                    overlapping += 1;
                } else if elf2.job_start <= elf1.job_start && elf2.job_end >= elf1.job_start {
                    overlapping += 1;
                }
            }
            None => println!("Error: {}. Your Regex fucked up.", line),
        }
    }

    return (contained, overlapping);
}
