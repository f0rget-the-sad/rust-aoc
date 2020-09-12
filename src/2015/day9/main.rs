extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/2015/day9/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    //assert_eq!(res1, 0);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    //assert_eq!(res2, 0);
}

fn part1(cont: &str) -> u16 {
    let mut cities = HashMap::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut idx;
    let mut len_matrix = vec![vec![0; 100]; 100];

    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        let (fromc, toc, len) = (
            // get by index(&caps[1]) gives
            // do not live long enough
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str().parse::<u16>().unwrap(),
        );
        idx = cities.len();
        cities.entry(fromc).or_insert(idx);
        idx = cities.len();
        cities.entry(toc).or_insert(idx);

        len_matrix[cities[fromc]][cities[toc]] = len;
        len_matrix[cities[toc]][cities[fromc]] = len;
    }
    idx = cities.len();
    let mut shortest_path = 65535;
    let mut longest_path = 0;
    for p in (0..idx).permutations(idx) {
        let mut iter = p.into_iter();
        let mut len = 0;
        let mut c1 = iter.next().unwrap();
        while let Some(c2) = iter.next() {
            len += len_matrix[c1][c2];
            c1 = c2;
        }
        shortest_path = cmp::min(shortest_path, len);
        longest_path = cmp::max(longest_path, len);
    }
    dbg!(shortest_path);
    dbg!(longest_path);
    shortest_path
}

fn part2(cont: &str) -> u16 {
    0
}
