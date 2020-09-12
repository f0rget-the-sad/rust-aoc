use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/<YEAR>/day<DAY>/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    //assert_eq!(res1, 0);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    //assert_eq!(res2, 0);
}

fn part1(cont: &str) -> u32 {
    0
}

fn part2(cont: &str) -> u32 {
    0
}
