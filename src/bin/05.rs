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

// Get mutable references to the elements at the two indices.
//     If `i != j`, returns `Ok(_)` with two references.
//     If `i == j`, returns `Err(_)` with a single reference.
//     If `i` or `j` is out of bounds, this panics.
fn get2<T>(slice: &mut [T], i: usize, j: usize) -> Result<(&mut T, &mut T), &mut T> {
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        Ok((&mut left[i], &mut right[0]))
    } else if j < i {
        let (left, right) = slice.split_at_mut(i);
        Ok((&mut left[j], &mut right[0]))
    } else {
        Err(&mut slice[i])
    }
}


fn e1(input: std::str::Split<&str>)
{
    let mut stack_crates: [VecDeque<char>; 10] = Default::default();         // NOTE:    no puedo hacer el array de tamaño size_stack :S

    let mut empty_line = false;
    for line in input
    {
        // PRINT (on empty line)
        if line == ""               // skip empty line
        {   
            empty_line = true;    

            println!("\nCrates arrangement:");
            for i in 0..stack_crates.len()
            {
                println!("{:?}", stack_crates[i]);
            }
        }
        
        else
        {
            // INPUT
            if !empty_line
            {
                let mut read_stack = 1;
                let mut read_crate; // char

                let mut linechars = line.chars().skip(1);

                // first it
                read_crate = linechars.next().unwrap();
                if read_crate != ' ' && !read_crate.is_numeric()
                {
                    stack_crates[read_stack].push_front(read_crate);
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
                // parse
                let mut linechars = line.chars().skip(5);           // skip "move "
                let mut times = String::from("");
                times.push(linechars.next().unwrap());
                
                let mut times_figures = linechars.next().unwrap();
                while times_figures.is_digit(10)
                {
                    times.push(times_figures);
                    times_figures = linechars.next().unwrap();
                }
                let times = times.parse::<usize>().unwrap();

                let mut linechars = linechars.skip(5);              // skip "from "
                let index_from = linechars.next().unwrap().to_digit(10);
                let mut linechars = linechars.skip(4);              // skip " to "
                let index_to = linechars.next().unwrap().to_digit(10);

                if index_from == None || index_to == None  {   break;  }
                // if times == None || index_from == None || index_to == None  {   break;  }
                // let times = times.unwrap() as usize;
                let index_from = index_from.unwrap() as usize;
                let index_to = index_to.unwrap() as usize;

                // destructure
                // move_crates(&mut stack_crates[index_from], &mut stack_crates[index_to], times);
                //  WARN:   cannot borrow `stack_crates[_]` as mutable more than once at a time   
                //              Solucion -> Destructuring:
                let (from, to);
                if index_from > index_to    {   (to, from) = get2(&mut stack_crates, index_from, index_to).unwrap();    }
                else                        {   (from, to) = get2(&mut stack_crates, index_from, index_to).unwrap();    }

                println!("from {:?} to {:?} {} times", from, to, times);                // TEST
                move_crates(from, to, times);
            }
        }
    }
}

fn main()
{
    let input = include_str!("../../input/05").split("\n");     // Read input

    e1(input);
}
