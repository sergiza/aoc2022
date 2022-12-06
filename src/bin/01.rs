use std::io::*;
use std::fs::*;

fn e1(line: std::str::Split<char>)
{
    let mut c_current = 0;
    let mut c_max = 0;

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
}

fn e2(line: std::str::Split<char>)
{
    let mut c_current = 0;
    let mut c_max_1 = 0;
    let mut c_max_2 = 0;
    let mut c_max_3 = 0;

    for value in line
    {
        if value != "" {
            c_current = c_current + value.parse::<i32>().unwrap();
        }
        else
        {
            if c_current > c_max_1 {
                c_max_3 = c_max_2;
                c_max_2 = c_max_1;
                c_max_1 = c_current;
            }
            else if c_current > c_max_2 {
                c_max_3 = c_max_2;
                c_max_2 = c_current;
            } 
            else if c_current > c_max_3 {
                c_max_3 = c_current;
            }
            println!("{} > {} > {} \t; {} current", c_max_1, c_max_2, c_max_3, c_current);
            c_current = 0;
        }

        println!("{} (+{})", c_current, value);
    }
    let total = c_max_1 + c_max_2 + c_max_3;
    println!("Total calories carried by the top 3 elves: {}", total);
}

fn main()
{
    // READ
    let mut file = File::open("../../input/01").expect("Can't open file!");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read file!");
    let line = text.split('\n');

    
    // e1(line);
    e2(line);
}
