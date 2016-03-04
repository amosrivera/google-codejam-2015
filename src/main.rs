use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let contents = read_input("input.txt");
    let mut lines: Vec<&str> = contents.split_terminator("\n").collect();
    lines.remove(0); // number of cases is not used

    for case in lines.chunks(2) {
        get_served_by(
            case[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap(),
            case[1].split(" ").map(|s| s.parse().unwrap()).collect()
        );
    }
}

fn get_served_by(queue: u32, barbers: Vec<u32>){
    let min  = barbers.iter().min().unwrap();
    let freq = barbers.iter().map(|&b| (b / min));
//    println!("{}",min);
    for barber in barbers {
        println!("{}", barber)
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
