use md5::{Digest, Md5};
use std::fs;

fn main() {
    let filename = "src/2015/day4/input";
    let mut cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    cont.pop();

    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 282749);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 9962624);
}

fn part1(cont: &str) -> u64 {
    let bcont = cont.as_bytes();
    for i in 0..std::u64::MAX {
        let hash = Md5::new()
            .chain(bcont)
            .chain(i.to_string().as_bytes())
            .finalize();
        if hash[..2] == [0, 0] && hash[2] <= 0x0F {
            println!("Hash: {:x}", hash);
            return i;
        }
    }
    0
}

fn part2(cont: &str) -> u64 {
    let bcont = cont.as_bytes();
    //TODO: use threads
    for i in 0..std::u64::MAX {
        let hash = Md5::new()
            .chain(bcont)
            .chain(i.to_string().as_bytes())
            .finalize();
        if hash[..3] == [0, 0, 0] {
            println!("Hash: {:x}", hash);
            return i;
        }
    }
    0
}
