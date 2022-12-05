use std::io::*;
use std::fs::*;

fn main() {
    let mut c_current = 0;
    let mut c_max = 0;

    // READ
    let mut file = File::open("../../input/01").expect("Can't open file!");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read file!");
    let line = text.split('\n');

    for value in line
    {
        if value != "" {
            c_current = c_current + value.parse::<i32>().unwrap();
        } else {
            c_current = 0;
        }

        if c_current > c_max {
            c_max = c_current;
        }

        println!("{} (+{}) \tMAX={}", c_current, value, c_max);
    }
    println!("Max calories: {}", c_max);
}
