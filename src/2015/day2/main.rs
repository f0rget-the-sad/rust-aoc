use std::fs;

fn main() {
    let filename = "src/2015/day2/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 1598415);
    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res1, 3812909);
}

fn part1(cont: &str) -> u32 {
    let mut s: u32 = 0;
    for line in cont.lines() {
        let dim: Vec<u32> = line.split("x").map(|d| d.parse().unwrap()).collect();
        let sides: Vec<u32> = vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]];
        //make this in one iter?
        s += sides.iter().map(|d| 2 * d).sum::<u32>();
        s += sides.iter().min().unwrap();
    }
    s
}

fn part2(cont: &str) -> u32 {
    let mut s: u32 = 0;
    for line in cont.lines() {
        let dim: Vec<u32> = line.split("x").map(|d| d.parse().unwrap()).collect();
        let perimeters: Vec<u32> = vec![
            2 * (dim[0] + dim[1]),
            2 * (dim[0] + dim[2]),
            2 * (dim[1] + dim[2]),
        ];
        //make this in one iter?
        s += perimeters.iter().min().unwrap();
        s += dim.iter().product::<u32>();
    }
    s
}
