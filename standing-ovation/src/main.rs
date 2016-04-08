use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let contents = read_input("input.txt");
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut results = Vec::new();
    lines.remove(0); // number of cases is not used

    for case in lines {
        results.push(
            get_count(
                case
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect()
            )
        )
    }

    save(results);
}

fn get_count(audience: Vec<u32>) -> u32 {
    let mut total   = 0;
    let mut friends = 0;

    for (shyness, people) in audience.into_iter().enumerate() {
        if (people > 0) && ((total + friends) < (shyness as u32)) {
            friends += shyness as u32 - (total + friends);
        }

        total += people;
    }

    return friends;
}

fn save(results: Vec<u32>) {
    let mut options = OpenOptions::new();
    let path = Path::new("output.txt");
    options.write(true);

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error creating file"),
    };

    for (i, result) in results.iter().enumerate() {
        file.write(
            format!("Case #{}: {}\n", i + 1, result).as_bytes()
        );
    }
}

fn read_input(file_name: &'static str) -> String {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(_) => panic!("Couldn't read file"),
        Ok(s) => s,
    };

    return s;
}
