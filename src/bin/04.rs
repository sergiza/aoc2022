#[derive(Copy, Clone)]
struct Pair
{
    first:  i32,
    last:   i32,
}

impl Pair
{
    fn contains(&mut self, other_elf: Pair) -> bool
    {
        if self.first <= other_elf.first && self.last >= other_elf.last
        {
            return true
        }
        return false
    }
}


fn e1(input: std::str::Split<&str>)
{
    let mut c = 0;
    for line in input
    {
        if line != ""
        {
            let groups = line.split(",");
    
            let mut elves: [Pair; 2] = [ Pair { first:0, last:0 }, Pair { first:0, last:0 } ];
            let mut i = 0;
            for group in groups
            {
                let mut number = group.split("-");
                elves[i].first = number.nth(0).expect("Error enunciado?").parse::<i32>().unwrap();
                elves[i].last  = number.nth(0).expect("Error enunciado?").parse::<i32>().unwrap();
    
                i = i + 1;
            }

            // CONTAINS
            if elves[0].contains(elves[1]) || elves[1].contains(elves[0])
            {
                c = c + 1;
                println!("Does contain:\t\t{}\t\t{}", line, c);
            }
            else
            {
                println!("Does NOT contain:\t{}\t\t{}", line, c);
            }
        }
    }
}

fn main()
{
    let input = include_str!("../../input/04").split("\n");     // Read input

    e1(input);
}
