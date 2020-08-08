use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/2015/day3/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 2572);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 2631);
}

fn part1(cont: &str) -> usize {
    let mut houses = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    houses.insert((x, y), true);

    for c in cont.chars() {
        do_move(c, &mut x, &mut y);
        houses.entry((x, y)).or_insert(true);
    }
    houses.len()
}

fn part2(cont: &str) -> usize {
    let mut houses = HashMap::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut is_robo = false;
    houses.insert((sx, sy), true);

    for c in cont.chars() {
        if is_robo {
            do_move(c, &mut rx, &mut ry);
            houses.entry((rx, ry)).or_insert(true);
        } else {
            do_move(c, &mut sx, &mut sy);
            houses.entry((sx, sy)).or_insert(true);
        }

        is_robo = !is_robo;
    }
    houses.len()
}

fn do_move(c: char, x: &mut i32, y: &mut i32) {
    match c {
        '^' => *y += 1,
        'v' => *y -= 1,
        '>' => *x += 1,
        '<' => *x -= 1,
        _ => println!("WTF is this"),
    }
}
