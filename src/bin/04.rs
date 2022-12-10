#[derive(Copy, Clone)]
struct Pair
{
    first:  i32,
    last:   i32,
}

impl Pair
{
    fn contains(&self, other_elf: &Pair) -> bool
    {
        if self.first <= other_elf.first && self.last >= other_elf.last
        {
            return true
        }
        return false
    }
    fn overlaps(&mut self, other_elf: Pair) -> bool
    {
        if self.first <= other_elf.first && other_elf.first <= self.last
        {
            return true
        }
        if self.first <= other_elf.last && other_elf.last <= self.last
        {
            return true
        }
        return false
    }
}


fn e1(input: std::str::Split<&str>)
{
    let mut c_contains = 0;
    let mut c_overlaps = 0;

    for line in input
    {
        if line != ""
        {
            let group = line.split(",");
    
            let mut elves: [Pair; 2] = [ Pair { first:0, last:0 }, Pair { first:0, last:0 } ];
            let mut i = 0;
            for range in group
            {
                // cleaner version - utilizando split_once
                let (first, last) = range.split_once("-").unwrap();
                elves[i].first = first.parse::<i32>().unwrap();
                elves[i].last = last.parse::<i32>().unwrap();

                // old version
                // let mut number = range.split("-");
                // elves[i].first = number.nth(0).expect("INPUT FILE UNEXPECTED?").parse::<i32>().unwrap();
                // elves[i].last  = number.nth(0).expect("INPUT FILE UNEXPECTED?").parse::<i32>().unwrap();
    
                i = i + 1;
            }

            // CONTAINS
            if elves[0].contains(&elves[1]) || elves[1].contains(&elves[0])
            {
                c_contains = c_contains + 1;
                print!("{}\tDoes contain \t\t({})", line, c_contains);
            }
            else
            {
                print!("{}\t...\t\t\t", line);
            }

            // OVERLAPS
            if elves[0].overlaps(elves[1]) || elves[1].overlaps(elves[0])
            {
                c_overlaps = c_overlaps + 1;
                print!("\t\t Overlaps \t\t{}", c_overlaps);
            }
            println!();
        }
    }
}

fn main()
{
    let input = include_str!("../../input/04").split("\n");     // Read input

    e1(input);
}
