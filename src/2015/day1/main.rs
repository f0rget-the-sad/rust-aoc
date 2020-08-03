use std::fs;

fn main() {
    println!("part1 :\n{}", part1());
    println!("part2 :\n{}", part2());
}

fn part1() -> i32 {
    let filename = "src/2015/day1/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut count = 0;
    for c in cont.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }
    }
    count
}

fn part2() -> i32 {
    let filename = "src/2015/day1/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut count = 0;
    let mut pos = 1;
    for c in cont.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            count -= 1;
        }
        if count == -1 {
            break;
        }
        pos += 1;
    }
    pos
}
