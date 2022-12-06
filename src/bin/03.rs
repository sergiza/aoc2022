fn offset(chara: char) -> i32
{
    if chara.is_uppercase() {  return chara as i32 - 38 }
    else                    {  return chara as i32 - 96 }
}

fn e1(input: std::str::Split<&str>)
{
    let mut repeating: char = 'a';
    let mut sum = 0;

    for line in input
    {
        let halves = line.split_at(line.len()/2);

        for c in halves.0.chars() {   
            if halves.1.contains(c) {
                repeating = c;
            }
        }

        if repeating.is_uppercase() {   sum = sum + offset(repeating)   }   // caps
        else                        {   sum = sum + offset(repeating)   }   // lower

        println!("{}\t{} -> {} | {}", sum, repeating, halves.0, halves.1);
    }
}

fn e2(input: std::str::Split<&str>)
{
    let mut repeating: char = 'a';
    let mut sum = 0;

    let mut group = 0;
    let mut line_0 = "";
    let mut line_1 = "";

    for line in input
    {
        match group
        {
            0 => 
            {
                line_0 = line;
                group = group + 1;
            },
            1 => 
            {
                line_1 = line;
                group = group + 1;
            },
            2 => 
            {

                for c in line.chars() {   
                    if line_0.contains(c) {
                        if line_1.contains(c) {
                            repeating = c;
                        }
                    }
                }

                sum = sum + offset(repeating);
                println!("{}\t{} -> {} & {} & {}", sum, repeating, line_0, line_1, line);
                group = 0;
            },
            _ => group = 0,
        }
    }
}

fn main()
{
    let input = include_str!("../../input/03").split("\n");     // Read input

    // e1(input);
    e2(input);
}
