// fn size_stack(input: std::str::Split<&str>) -> usize
// {
//     // while != "" 
//     //  guardar last_line
//     // return last_line.ultimo
//     return 10
// }

fn move_crates(from: &mut Vec<char>, to: &mut Vec<char>, mut times: usize)
{
    while times != 0
    {
        to.push(from.pop().unwrap());
        times = times -1;
    }
}

fn e1(input: std::str::Split<&str>)
{
    let mut stack_crates: [Vec<char>; 10] = Default::default();         // NOTE:    no puedo hacer el array de tamaño size_stack :S
    stack_crates[1].push('Z');
    stack_crates[1].push('N');
    stack_crates[2].push('M');
    stack_crates[2].push('C');
    stack_crates[2].push('D');
    stack_crates[3].push('P');

    // INPUT
    for line in input
    {
        while line != ""
        {
            let read_stack = 0;
            let read_crate = 'Z';
            stack_crates[read_stack].push(read_crate);
        }
    }

    // MOVE
    // move_crates(&mut stack_crates[1], &mut stack_crates[2], 1);
    //  WARN:   cannot borrow `stack_crates[_]` as mutable more than once at a time   
    //              Solucion -> Destructuring:
    let [_, from, to, ..] = &mut stack_crates;
    move_crates(from, to, 1);


    // Print
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
