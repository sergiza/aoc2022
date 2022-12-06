fn rock_paper_scissors(elf:i32, me:i32) -> i32
{
    if elf == 1
    {
        if me == 2  { return 6  }
        if me == 3  { return 0  }
    }
    if elf == 2
    {
        if me == 3  { return 6  }
        if me == 1  { return 0  }
    }
    if elf == 3
    {
        if me == 1  { return 6  }
        if me == 2  { return 0  }
    }

    return 3
}

fn rock_paper_scissors_inverse(elf:i32, result:i32) -> i32
{
    if elf == 1
    {
        if result == 6  { return 2  }
        if result == 3  { return 1  }
        if result == 0  { return 3  }
    }
    if elf == 2
    {
        if result == 6  { return 3  }
        if result == 3  { return 2  }
        if result == 0  { return 1  }
    }
    if elf == 3
    {
        if result == 6  { return 1  }
        if result == 3  { return 3  }
        if result == 0  { return 2  }
    }

    return -1
}

fn main()
{
    // READ
    let input = include_str!("../../input/02").split("\n");

    let mut score = 0;
    let mut elf = 0;
    let mut me = 0;

    // EJ 1
    // for line in input
    // {
    //     if line != ""
    //     {
    //         let round = line.split(" ");
    //         for hand in round
    //         {
    //             if hand.starts_with('A') { elf = 1; }
    //             if hand.starts_with('B') { elf = 2; }
    //             if hand.starts_with('C') { elf = 3; }
    //             if hand.starts_with('X') { me = 1; }
    //             if hand.starts_with('Y') { me = 2; }
    //             if hand.starts_with('Z') { me = 3; }
    //         }
    //         score = score + me + rock_paper_scissors(elf, me);
    //         println!("SCORE = {}\t<-\t{}", score, line);
    //     }
    // }

    // EJ 2
    let mut result = 0;
    for line in input
    {
        if line != ""
        {
            let round = line.split(" ");
            for hand in round
            {
                if hand.starts_with('A') { elf = 1; }
                if hand.starts_with('B') { elf = 2; }
                if hand.starts_with('C') { elf = 3; }
                if hand.starts_with('X') { result = 0; }
                if hand.starts_with('Y') { result = 3; }
                if hand.starts_with('Z') { result = 6; }
            }
            me = rock_paper_scissors_inverse(elf, result);
            score = score + me + result;
            println!("SCORE = {}\t<-\t{}", score, line);
        }
    }
}
