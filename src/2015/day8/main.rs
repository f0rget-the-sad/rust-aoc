use std::fs;

fn main() {
    let filename = "src/2015/day8/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 1342);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 2074);
}

fn part1(cont: &str) -> u32 {
    let mut n_of_code: u32 = 0;
    let mut n_of_mem: u32 = 0;
    for line in cont.lines() {
        n_of_code += line.len() as u32;
        n_of_mem += line.len() as u32 - 2; // 2 for start and end double quote
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next().unwrap() {
                    '\\' | '\"' => n_of_mem -= 1,
                    'x' => {
                        chars.next().unwrap();
                        chars.next().unwrap();
                        n_of_mem -= 3;
                    }
                    _ => panic!("Unexpected char"),
                }
            }
        }
    }
    n_of_code - n_of_mem
}

fn part2(cont: &str) -> u32 {
    let mut n_of_code: u32 = 0;
    let mut n_of_mem: u32 = 0;
    for line in cont.lines() {
        n_of_code += line.len() as u32;
        n_of_mem += line.len() as u32 + 4; // 4 is "" encodes to "\"\""
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next().unwrap() {
                    '\\' | '\"' => n_of_mem += 2,
                    'x' => {
                        chars.next().unwrap();
                        chars.next().unwrap();
                        n_of_mem += 1;
                    }
                    _ => panic!("Unexpected char"),
                }
            }
        }
    }
    n_of_mem - n_of_code
}
