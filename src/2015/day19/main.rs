use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "src/2015/day19/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 535);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 212);
}

fn part1(cont: &str) -> u32 {
    let mut is_molecule = false;
    let mut replacements = HashMap::<&str, Vec<&str>>::new();
    let mut distinct_molecules = HashSet::new();
    for l in cont.lines() {
        if l.is_empty() {
            is_molecule = true;
        }
        if !is_molecule {
            // collect  replacements
            let r: Vec<&str> = l.split_terminator(" => ").collect();
            if let Some(val) = replacements.get_mut(r[0]) {
                val.push(r[1])
            } else {
                replacements.insert(r[0], vec![r[1]]);
            }
        } else {
            // parse molecule
            let mut el = String::new();
            for (n, c) in l.chars().enumerate() {
                if !c.is_lowercase() {
                    if el.is_empty() {
                        el.push(c);
                        continue;
                    }
                    if let Some(values) = replacements.get(el.as_str()) {
                        for v in values {
                            let mut cur_molecule = l.to_string();
                            cur_molecule.replace_range(n-el.len()..n, v);
                            distinct_molecules.insert(cur_molecule);
                        }
                    }
                    el.clear();
                }
                el.push(c);
            }
            // handle last element
            if let Some(values) = replacements.get(el.as_str()) {
                for v in values {
                    let mut cur_molecule = l.to_string();
                    cur_molecule.replace_range(l.len()-el.len()..l.len(), v);
                    distinct_molecules.insert(cur_molecule);
                }
            }
        }
    }
    distinct_molecules.len() as u32
}

fn part2(cont: &str) -> u32 {
    let mut is_molecule = false;
    let mut hmap = HashMap::<&str, &str>::new();
    for l in cont.lines() {
        if l.is_empty() {
            is_molecule = true;
            continue;
        }
        if !is_molecule {
            // collect  replacements
            let r: Vec<&str> = l.split_terminator(" => ").collect();
            if hmap.contains_key(r[1]) {
                unimplemented!("Duplicate results is not supported");
            } else {
                hmap.insert(r[1], r[0]);
            }
        } else {
            // WARNING: I was just trying to solve it starting from
            // dummies solution and then improve it to more optimized,
            // but the first implementation actually solve it
            // and I loose motivation to improve my solution

            let mut molecules : String = l.to_string();
            let mut count = 0;
            let mut steps = 0;
            loop {
                let before : String = molecules.clone();
                for (key, val) in hmap.iter() {
                    // I with there will be a version -> Some(String) to
                    // indicate if it does something with string
                    let new = molecules.replacen(key, val, 1);
                    if new != molecules {
                        steps += 1;
                        molecules = new;
                    }
                }
                if molecules == "e" {
                    return steps
                }
                if molecules == before {
                    panic!("no more replacements avail {}!", count);
                }
                count += 1;
            }
        }
    }
    0
}
