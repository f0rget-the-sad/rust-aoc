use std::cmp;
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let filename = "src/2015/day13/input";
    //let filename = "src/2015/day13/test_input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    //assert_eq!(res1, 0);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    //assert_eq!(res2, 0);
}

fn part1(cont: &str) -> i32 {
    let mut happ_map: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    //Alice would gain 2 happiness units by sitting next to Bob.
    let re =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)").unwrap();

    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        let (who, sign, number, whom) = (
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str(),
        );
        //dbg!(who, sign, number, whom);
        happ_map.entry(who).or_insert(HashMap::new());
        // Currently, HashMap does not implement IndexMut, while Vec does,
        let sign: i32 = match sign {
            "gain" => 1,
            "lose" => -1,
            _ => panic!(),
        };
        happ_map.get_mut(who).unwrap().insert(whom, sign * number);
    }

    let mut best_score = 0;
    for table in happ_map.keys().permutations(happ_map.len()) {
        let mut score = 0;
        for i in 0..table.len() {
            // wish it could be smart enough to check that
            // (0 - 1) case is handled later in code...
            let mut prev = i.wrapping_sub(1);
            let mut next = i + 1;

            if i == 0 {
                prev = table.len() - 1;
            } else if i == (table.len() - 1) {
                next = 0;
            }
            score += happ_map[table[i]][table[next]];
            score += happ_map[table[i]][table[prev]];
        }
        best_score = cmp::max(best_score, score);
    }
    best_score
}

fn part2(cont: &str) -> i32 {
    let mut happ_map: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    //Alice would gain 2 happiness units by sitting next to Bob.
    let re =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)").unwrap();

    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        let (who, sign, number, whom) = (
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str(),
        );
        //dbg!(who, sign, number, whom);
        happ_map.entry(who).or_insert(HashMap::new());
        // Currently, HashMap does not implement IndexMut, while Vec does,
        let sign: i32 = match sign {
            "gain" => 1,
            "lose" => -1,
            _ => panic!(),
        };
        happ_map.get_mut(who).unwrap().insert(whom, sign * number);
    }

    happ_map.entry("me").or_insert(HashMap::new());
    // even later we wan't only mutate internal HT, it still equires a mutable borrow, so copy
    // keys to iterate later
    let keys = happ_map.keys().map(|k| k.clone()).collect::<Vec<&str>>();
    for person in keys {
        happ_map.get_mut("me").unwrap().insert(&person, 0);
        happ_map.get_mut(person).unwrap().insert("me", 0);
    }

    let mut best_score = 0;
    for table in happ_map.keys().permutations(happ_map.len()) {
        let mut score = 0;
        for i in 0..table.len() {
            // wish it could be smart enough to check that
            // (0 - 1) case is handled later in code...
            let mut prev = i.wrapping_sub(1);
            let mut next = i + 1;

            if i == 0 {
                prev = table.len() - 1;
            } else if i == (table.len() - 1) {
                next = 0;
            }
            score += happ_map[table[i]][table[next]];
            score += happ_map[table[i]][table[prev]];
        }
        best_score = cmp::max(best_score, score);
    }
    best_score
}
