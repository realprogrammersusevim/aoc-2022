use crate::utils::read_file;
use regex::Regex;

pub fn sim_stack() -> (String, String) {
    let input = read_file("input/5.input");
    let lines = input.lines();

    let mut stack1: Vec<Vec<String>> = Vec::new();
    let mut stack2: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for (i, line) in lines.enumerate() {
        if i < 9 {
            // Convert the Regex captures to a Vector
            let mut caps_vec: Vec<String> = Vec::new();
            for char in line.chars() {
                if char.is_alphabetic() {
                    caps_vec.push(char.to_string());
                }
            }
            stack1.push(caps_vec);
        } else {
            // If we're on the tenth line we've finished getting the stack
            if i + 1 == 10 {
                for vec in stack1.iter_mut() {
                    vec.reverse();
                }
                stack2 = stack1.clone();
            }
            // print!("{:?}", stack);
            let results = re.captures(line).unwrap();
            let amount = results[1].parse::<usize>().unwrap();
            let source = results[2].parse::<usize>().unwrap() - 1;
            let dest = results[3].parse::<usize>().unwrap() - 1;

            // println!("move {} from {} to {}", amount, source + 1, dest + 1);
            let mut temp2_vec: Vec<String> = Vec::new();
            for _i in 1..=amount {
                // Last item in the Vector is the top
                let temp1 = stack1[source].pop().unwrap();
                let temp2 = stack2[source].pop().unwrap();
                // println!("temp: {:?}", temp);
                temp2_vec.push(temp2);
                stack1[dest].push(temp1);
            }

            temp2_vec.reverse();
            stack2[dest].extend(temp2_vec);
            // println!("{:?}", stack);
        }
    }

    let lasts_vec1: Vec<String> = stack1.iter().map(|x| x[x.len() - 1].clone()).collect();
    let lasts1 = lasts_vec1.join("");

    let lasts_vec2: Vec<String> = stack2.iter().map(|x| x[x.len() - 1].clone()).collect();
    let lasts2 = lasts_vec2.join("");
    // assert_eq!(stack, vec![vec!["C"], vec!["M"], vec!["Z", "N", "D", "P"]]);
    (lasts1, lasts2)
}
