use std::{cmp, fs};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let file_path = &args[0];
    let file_content = read_file(file_path);
    let res = file_content.lines().fold(
        Container { max: 0, current: 0 },
        |mut acc: Container, x: &str| {
            if x.is_empty() {
                acc.max = cmp::max(acc.max, acc.current);
                acc.current = 0;
                return acc;
            }
            let parsed_num = x.parse::<u32>().unwrap();
            acc.current += parsed_num;
            acc
        },
    );
    println!("Max: {}", res.max);
}

fn read_file(path: &String) -> String {
    let content = fs::read_to_string(path);
    match content {
        Ok(f) => {
            return f;
        }
        Err(err) => {
            panic!("Error: {}", err)
        }
    }
}

struct Container {
    max: u32,
    current: u32,
}
