fn size_stack(input: std::str::Split<&str>) -> usize
{
    // while != "" 
    //  guardar last_line
    // return last_line.ultimo
    return 10
}

fn move_crates(from: &Vec<char>, to: &Vec<char>)
{

}

fn e1(input: std::str::Split<&str>)
{
    // for line in input
    // {
    //     println!("{}", line);
    // }

    let mut stack_crates: [Vec<char>; 10] = Default::default();     // NOTE: no puedo hacer el array de tamaño size_stack :S
    stack_crates[1].push('Z');
    stack_crates[1].push('N');
    stack_crates[2].push('M');
    stack_crates[2].push('C');
    stack_crates[2].push('D');
    stack_crates[3].push('P');

    let mut x = stack_crates[1].pop();
    while x.is_some()
    {
        println!("{:?}", x.unwrap());
        x = stack_crates[1].pop();
    }
}

fn main()
{
    let input = include_str!("../../input/05mini").split("\n");     // Read input

    e1(input);
}
