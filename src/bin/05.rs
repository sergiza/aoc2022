use std::collections::VecDeque;
// fn size_stack(input: std::str::Split<&str>) -> usize {
//     // while != "" 
//     //  guardar last_line
//     // return last_line.ultimo
//     return 10
// }

fn move_crates(from: &mut VecDeque<char>, to: &mut VecDeque<char>, mut times: usize)
{
    while times != 0
    {
        to.push_back(from.pop_back().unwrap());
        times = times -1;
    }
}

fn e1(input: std::str::Split<&str>)
{
    let mut stack_crates: [VecDeque<char>; 10] = Default::default();         // NOTE:    no puedo hacer el array de tamaño size_stack :S

    let mut empty_line = false;
    for line in input
    {
        if line == ""               // skip empty line
        {   
            empty_line = true;    
        }
        
        else
        {
            // INPUT
            if !empty_line
            {
                let mut read_stack = 1;
                let mut read_crate = ' ';

                let mut linechars = line.chars().skip(1);

                // first it
                read_crate = linechars.next().unwrap();
                if read_crate != ' ' && !read_crate.is_numeric()
                {
                    stack_crates[read_stack].push_front(read_crate);
                    // println!("FIRST IT:\t{:?}", stack_crates[read_stack]);           // TEST
                }

                // rest it
                let mut skip = 0;
                for char in linechars
                {
                    if skip == 3
                    {
                        read_stack = read_stack + 1;

                        read_crate = char;
                        if read_crate != ' ' && !read_crate.is_numeric()
                        {
                            stack_crates[read_stack].push_front(read_crate);
                            // println!("...:\t{:?}", stack_crates[read_stack]);        // TEST
                        }
                        skip = 0;
                    }
                    else
                    {
                        skip = skip + 1;
                    }
                }
            }

            // MOVE
            else
            {
                let mut linechars = line.chars().skip(5);
                let times = linechars.next().unwrap().to_digit(10).unwrap() as usize;

                // move_crates(&mut stack_crates[1], &mut stack_crates[2], 1);
                //  WARN:   cannot borrow `stack_crates[_]` as mutable more than once at a time   
                //              Solucion -> Destructuring:
                let [_, from, to, ..] = &mut stack_crates;
                println!("from {:?} to {:?} {} times", from, to, times);                // TEST
                move_crates(from, to, times);
            }
        }
    }

    // PRINT
    println!("\nAfter the rearrangement procedure completes:");
    for i in 0..stack_crates.len()
    {
        println!("{:?}", stack_crates[i]);
    }
}

fn main()
{
    let input = include_str!("../../input/05mini").split("\n");     // Read input

    e1(input);
}
