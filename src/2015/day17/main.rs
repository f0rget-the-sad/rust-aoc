use itertools::Itertools;
use std::fs;

const GOAL: usize = 150;

fn main() {
    let filename = "src/2015/day17/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 1638);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 17);
}


fn part1(cont: &str) -> u32 {
    let vec: Vec<u32> = cont.lines().map(|l|l.parse().unwrap()).collect();
    let len = vec.len();

    let mut res = 0;
    for i in 2..len {
        for vc in vec.iter().combinations(i) {
            if vc.iter().map(|&x|x).sum::<u32>() == GOAL as u32 {
                res += 1;
            }
        }
    }
    res
}

fn part2(cont: &str) -> u32 {
    let vec: Vec<u32> = cont.lines().map(|l|l.parse().unwrap()).collect();
    let len = vec.len();
    let mut results = vec![];
    let mut min_len = len;
    for i in 2..len {
        for vc in vec.iter().combinations(i) {
            if vc.iter().map(|&x|x).sum::<u32>() == GOAL as u32 {
                if min_len > vc.len() {
                    min_len = vc.len();
                }
                results.push(vc.len());
            }
        }
    }
    results.iter().filter(|&x| *x == min_len).count() as u32
}
