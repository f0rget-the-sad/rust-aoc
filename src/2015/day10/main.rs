use std::char;
use std::fs;

fn main() {
    let filename = "src/2015/day10/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont, 40);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 492982);

    let res2 = part1(&cont, 50);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 6989950);
}

fn part1(cont: &str, n: u32) -> u32 {
    let s = cont.trim_end();
    let mut end_s: Vec<char> = s.chars().collect();
    for _ in 0..n {
        let mut cur_s = Vec::new();
        let mut counter = 1;
        for i in 0..end_s.len() - 1 {
            if end_s[i] == end_s[i + 1] {
                counter += 1;
            } else {
                cur_s.push(char::from_digit(counter, 10).unwrap());
                cur_s.push(end_s[i]);
                counter = 1;
            }
        }
        cur_s.push(char::from_digit(counter, 10).unwrap());
        cur_s.push(end_s[end_s.len() - 1]);

        end_s = cur_s;
    }
    end_s.len() as u32
}
