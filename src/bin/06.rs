use std::collections::VecDeque;

fn check_marker(mut marker: &VecDeque<char>) -> bool
// fn check_marker(mut marker: &Vec<char>) -> bool
{
    println!("{:?}", marker);

    let aux = marker;
    for char in aux.iter().skip(1)
    {
        // println!("{},{}", marker[0], *char);
        // if marker[0] == *char
        // {
            println!("------------------ Return falsers! ");
            return false
        // }
    }
    println!("Return truers! ----------------");
    return true
}

fn e1(input: String) -> Option<VecDeque<char>>
{
    let mut input = input.chars();
    let mut marker: VecDeque<char> = Default::default();
    let window = 4;
    let mut c = window;

    // first it
    for i in 0..window
    {
        marker.push_back(input.next().unwrap());
    }

    if check_marker(&mut marker)            // check if marker
    {
        println!("Marker:\t{:?}\t -> {}", marker, c);
        return Some(marker)
    }

    // rest it
    for char in input
    {
        c = c + 1;
        marker.pop_back();
        marker.push_front(char);

        if check_marker(&mut marker)        // check if marker
        {
            println!("Marker:\t{:?}\t -> {}", marker, c);
            return Some(marker)
        }
    }
    return None
}

fn main()
{
    let input = include_str!("../../input/06mini").to_string();     // Read input

    e1(input);
    // e2(input);
}
