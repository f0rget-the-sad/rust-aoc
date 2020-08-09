use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "src/2015/day5/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);

    assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice2("xxyxx"), true);
    assert_eq!(is_nice2("aaa"), false);
    assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
}

fn part1(cont: &str) -> u32 {
    let mut count = 0;
    let hv: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
    let forbidden: HashSet<&str> = vec!["ab", "cd", "pq", "xy"].iter().cloned().collect();
    for line in cont.lines() {
        if is_nice(line, &hv, &forbidden) {
            count += 1;
        }
    }
    count
}

fn is_nice(line: &str, hv: &HashSet<char>, forbidden: &HashSet<&str>) -> bool {
    let mut vowels = 0;
    let mut pc = ' ';
    let mut is_dd = false;
    for c in line.chars() {
        if vowels < 3 {
            if hv.contains(&c) {
                vowels += 1;
            }
        }
        if !is_dd && c == pc {
            is_dd = true;
        }
        let s: String = [pc, c].iter().collect();
        if forbidden.contains::<str>(&s) {
            return false;
        }
        pc = c;
    }
    vowels >= 3 && is_dd
}

fn part2(cont: &str) -> u32 {
    let mut count = 0;
    for line in cont.lines() {
        if is_nice2(line) {
            count += 1;
        }
    }
    count
}

fn is_nice2(line: &str) -> bool {
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut is_pair = false;
    let mut is_between = false;
    let line: Vec<char> = line.chars().collect();

    for i in 1..line.len() {
        if i < line.len() - 1 {
            if line[i - 1] == line[i + 1] {
                is_between = true;
            }
        }

        if !is_pair {
            let pair: String = [line[i - 1], line[i]].iter().collect();
            match pairs.get(&pair) {
                Some(val) => {
                    //overlap check
                    if *val < i - 1 {
                        is_pair = true;
                    }
                }
                None => {
                    pairs.insert(pair, i);
                }
            }
        }

        if is_pair && is_between {
            return true;
        }
    }
    false
}
