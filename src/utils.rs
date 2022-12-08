use std::fs;

pub fn read_file(file_name: &str) -> String {
    

    match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading file: {}", err),
    }
}
