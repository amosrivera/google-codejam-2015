use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let contents = read_input("input.txt");
    let mut lines: Vec<&str> = contents.split_terminator("\n").collect();
    let mut solutions = Vec::new();
    let mut result;
    let mut c = 0;
    lines.remove(0); // number of cases is not used

    for case in lines.chunks(2) {
        c += 1;
        println!("Case: {}", c);
        result = get_served_by(
            case[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap(),
            case[1].split(" ").map(|s| s.parse().unwrap()).collect()
        );

        solutions.push(result);
    }

    save(solutions);
}

fn get_served_by(queue: u32, barbers: Vec<u32>) -> u32{
    let min  = barbers.iter().cloned().min().unwrap();

    let freq = barbers
                .iter()
                .cloned()
                .map(|b| (b as f32 / min as f32).ceil() as u32)
                .collect::<Vec<u32>>();

    let mut count = 0;

    for n in 0..queue {
        for (i,f) in freq.iter().enumerate() {
            if n % f == 0 {
                count += 1;
                if count == queue {
                    return (i+1) as u32;
                }
            }
        }
    }

    return 0;
}

fn save(solutions: Vec<u32>) {
    let mut options = OpenOptions::new();
    let path = Path::new("output.txt");
    options.write(true);

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error creating file"),
    };

    for (i,s) in solutions.iter().enumerate() {
        file.write(b"Case #");
        file.write((i+1).to_string().as_bytes());
        file.write(b": ");
        file.write(s.to_string().as_bytes());
        file.write(b"\n");
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
