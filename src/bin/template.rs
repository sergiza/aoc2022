fn e1(input: std::str::Split<&str>)
{
    for line in input
    {
        println!("{}", line);
    }
}

fn main()
{
    let input = include_str!("../../input/01mini").split("\n");     // Read input

    e1(input);
    // e2(input);
}
