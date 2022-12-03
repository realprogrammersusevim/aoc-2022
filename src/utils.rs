use std::fs;

pub fn read_file(file_name: &str) -> String {
    let file_contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading file: {}", err),
    };

    return file_contents;
}
