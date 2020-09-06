use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/2015/day7/input";
    //let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let cont = "123 -> x\n\
        456 -> y\n\
        x AND y -> d\n\
        x OR y -> e\n\
        x LSHIFT 2 -> f\n\
        y RSHIFT 2 -> g\n\
        NOT x -> h\n\
        NOT y -> i";
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    //assert_eq!(res1, 2572);

    //let res2 = part2(&cont);
    //println!("part2 :\n{}", res2);
    //assert_eq!(res2, 2631);
}

fn part1(cont: &str) -> usize {
    //Each wire can only get a signal from one source
    // so make source key in hash table
    //let mut houses = HashMap::new();
    for line in cont.lines() {
        println!("{}", line);
        let parts = line.split(" -> ");
    }
    //houses.insert((x, y), true);

    //for c in cont.chars() {
    //    do_move(c, &mut x, &mut y);
    //    houses.entry((x, y)).or_insert(true);
    //}
    //houses.len()
    0
}

fn parse_signal() {
    // 1 - number or wire
    // 2 - NOT and 1
    // 3 - 1 cmd 1
}
