use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/2015/day16/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 103);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 405);
}

/*
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
*/

fn match_key(key: &str, val: u32) -> bool {
    match key {
        "children" if val == 3 => true,
        "cats" if val == 7 => true,
        "samoyeds" if val == 2 => true,
        "pomeranians" if val == 3 => true,
        "akitas" if val == 0 => true,
        "vizslas" if val == 0 => true,
        "goldfish" if val == 5 => true,
        "trees" if val == 3 => true,
        "cars" if val == 2 => true,
        "perfumes" if val == 1 => true,
        _ => false,
    }
}

fn match_key2(key: &str, val: u32) -> bool {
    match key {
        "children" if val == 3 => true,
        "cats" if val > 7 => true,
        "samoyeds" if val == 2 => true,
        "pomeranians" if val < 3 => true,
        "akitas" if val == 0 => true,
        "vizslas" if val == 0 => true,
        "goldfish" if val < 5 => true,
        "trees" if val > 3 => true,
        "cars" if val == 2 => true,
        "perfumes" if val == 1 => true,
        _ => false,
    }
}

fn parser(cont: &str, match_f: &dyn Fn(&str, u32) -> bool) -> u32 {
    // Sue 8: akitas: 10, vizslas: 9, children: 3
    'outer: for l in cont.lines() {
        if let Some((sue, compounds)) = l.split_once(':') {
            let n = sue.split(' ').nth(1).unwrap();
            for comp in compounds.split(',') {
                let mut comp_split = comp.split(':');
                let key = comp_split.next().unwrap().strip_prefix(" ").unwrap();
                let val : u32 = comp_split.next().unwrap().strip_prefix(" ").unwrap().parse().unwrap();
                if !match_f(key, val) {
                    continue 'outer;
                }
            }
            return n.parse().unwrap();
        }
    }
    0
}

fn part1(cont: &str) -> u32 {
    parser(cont, &match_key)
}

fn part2(cont: &str) -> u32 {
    parser(cont, &match_key2)
}
