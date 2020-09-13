use std::fs;

fn main() {
    let filename = "src/2015/day11/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);

    println!("part1 :\n{}", res1);
    assert_eq!(res1, "hxbxxyzz");

    let res2 = part2(&res1);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, "hxcaabcc");
}

fn part1(cont: &str) -> String {
    let mut pass: Vec<char> = cont.trim_end().chars().collect();
    while !validate(&pass) {
        next_pass(&mut pass);
    }
    pass.into_iter().collect()
}

fn part2(cont: &str) -> String {
    let mut pass: Vec<char> = cont.trim_end().chars().collect();
    next_pass(&mut pass);
    part1(&pass.into_iter().collect::<String>())
}

fn next_pass(pass: &mut Vec<char>) {
    for i in (0..pass.len()).rev() {
        if pass[i] == 'z' {
            pass[i] = 'a';
        } else {
            pass[i] = (pass[i] as u8 + 1) as char;
            break;
        }
    }
}

fn validate(pass: &Vec<char>) -> bool {
    let mut straight_three = false;
    let mut pair1 = 0;
    let mut pair2 = 0;
    for i in 0..pass.len() {
        let c = pass[i];
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if i == pass.len() - 1 {
            break;
        }

        if i != 0 {
            if ((c as u8) - 1 == pass[i - 1] as u8) && (c as u8 + 1 == pass[i + 1] as u8) {
                straight_three = true;
            }
        }
        if c == pass[i + 1] {
            if pair1 == 0 {
                pair1 = i + 1;
            } else if pair2 == 0 && i > pair1 {
                pair2 = i;
            }
        }
    }
    return straight_three && pair1 != 0 && pair2 != 0;
}
