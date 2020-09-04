use std::fs;

fn main() {
    let filename = "src/2015/day6/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let res1 = part1(&cont);
    assert_eq!(res1, 377891);
    println!("part1 :\n{}", res1);

    let res2 = part2(&cont);
    assert_eq!(res2, 14110788);
    println!("part2 :\n{}", res2);
}

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

fn part1(cont: &str) -> u32 {
    let mut lights = vec![vec![false; 1000]; 1000];
    for line in cont.lines() {
        let mut parts = line.split_whitespace();
        let action = match parts.next().unwrap() {
            "toggle" => Action::Toggle,
            _ => match parts.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!(),
            },
        };

        let (startx, starty) = get_coords(parts.next().unwrap());

        //skip through
        parts.next().unwrap();

        let (endx, endy) = get_coords(parts.next().unwrap());

        for x in startx..endx + 1 {
            for y in starty..endy + 1 {
                match action {
                    Action::Toggle => lights[x][y] = !lights[x][y],
                    Action::On => lights[x][y] = true,
                    Action::Off => lights[x][y] = false,
                }
            }
        }
    }

    let mut sum = 0;
    for xyes in lights.iter() {
        for e in xyes.iter() {
            sum += *e as u32;
        }
    }
    sum
}

fn part2(cont: &str) -> u32 {
    let mut lights = vec![vec![0; 1000]; 1000];
    for line in cont.lines() {
        let mut parts = line.split_whitespace();
        let action = match parts.next().unwrap() {
            "toggle" => Action::Toggle,
            _ => match parts.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!(),
            },
        };

        let (startx, starty) = get_coords(parts.next().unwrap());

        //skip through
        parts.next().unwrap();

        let (endx, endy) = get_coords(parts.next().unwrap());

        for x in startx..endx + 1 {
            for y in starty..endy + 1 {
                match action {
                    Action::Toggle => lights[x][y] += 2,
                    Action::On => lights[x][y] += 1,
                    Action::Off => {
                        if lights[x][y] > 0 {
                            lights[x][y] -= 1
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for xyes in lights.iter() {
        sum += xyes.iter().sum::<u32>();
    }
    sum
}

fn get_coords(parts: &str) -> (usize, usize) {
    let numbers: Vec<usize> = parts.split(',').map(|s| s.parse().unwrap()).collect();
    (numbers[0], numbers[1])
}
