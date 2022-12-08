use crate::utils::read_file;

pub fn find_begin(length: i32) -> i32 {
    let input = read_file("input/6.input");
    // let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    let inter = input.chars().collect::<Vec<char>>();
    let windows = inter.windows(length.try_into().unwrap());

    for (i, window) in windows.enumerate() {
        // Check if any of the characters are the same
        let mut questionable: Vec<String> = Vec::new();
        for item in window {
            questionable.push(item.to_string());
        }
        let before = questionable.len();
        questionable.sort();
        questionable.dedup();

        if before == questionable.len() {
            let to_return = i + questionable.len();
            return to_return as i32;
        }
    }
    0
}
