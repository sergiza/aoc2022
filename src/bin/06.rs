use std::collections::VecDeque;

fn check_marker(marker: &VecDeque<char>) -> bool
{
    print!("{:?}", marker);

    let aux = marker;
    let mut i_marker = 0;
    let mut i_aux = 0;

    for char_marker in marker.iter()
    {
        for char_aux in aux
        {
            if i_marker != i_aux    // don't compare a char with itself
            {
                if char_marker == char_aux
                {
                    println!("\tfalse {} {}", char_marker, char_aux);
                    return false
                }
            }
            i_aux = i_aux + 1;
        }
        i_aux = 0;
        i_marker = i_marker + 1;
    }

    println!("\ttrue");
    return true
}

fn e1(input: String) -> Option<VecDeque<char>>
{
    let mut input = input.chars();
    let mut marker: VecDeque<char> = Default::default();

    // let window = 4;     // NOTE: EJ 1
    let window = 14;     // NOTE: EJ 2

    let mut c = window;

    // first it
    for _i in 0..window
    {
        marker.push_back(input.next().unwrap());
    }

    if check_marker(&marker)            // check if marker
    {
        println!("Marker:\t{:?}\t -> {}", marker, c);
        return Some(marker)
    }

    // rest it
    for char in input
    {
        c = c + 1;
        marker.pop_front();
        marker.push_back(char);

        if check_marker(&marker)        // check if marker
        {
            println!("Marker:\t{:?}\t -> {}", marker, c);
            return Some(marker)
        }
    }
    return None
}

fn main()
{
    let input = include_str!("../../input/06").to_string();     // Read input

    e1(input);
}
