use std::fs;

fn main() {
    let filename = "src/2015/day6/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    //println!("part1 :\n{}", res1);

    //let res2 = part2(&cont);
    //println!("part2 :\n{}", res2);
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
        println!("{}", line);
        //let parts: Vec<_> = line.split_whitespace().collect();
        let mut parts = line.split_whitespace();
        //let action = ;
        //for p in &parts {
        //    println!("{}", p);
        //}
        let action = match parts.next().unwrap() {
            "toggle" => Action::Toggle,
            _ => match parts.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!(),
            },
        };
        let (startx, starty) = parts.next().unwrap().split(',');
        println!("{:?}", action);

        break;
    }
    0
}
