//use std::collections::HashMap;

use regex::Regex;
use std::cmp;
use std::fs;

fn main() {
    let filename = "src/2015/day14/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 2696);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 1084);
}

const TOTAL_TIME: u32 = 2503;

fn part1(cont: &str) -> u32 {
    //let time = 1000;
    //Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
    let re =
        Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)")
            .unwrap();
    let mut winner: u32 = 0;
    for l in cont.lines() {
        //let test = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let caps = re.captures(l).unwrap();
        //let caps = re.captures(test).unwrap();
        let (_name, speed, speed_time, rest_time) = (
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        );
        let tt = speed_time + rest_time;
        let mut dist = (TOTAL_TIME / tt) * (speed * speed_time);
        let r = TOTAL_TIME % tt;
        if r <= speed_time {
            dist += r * speed;
        } else {
            dist += speed_time * speed
        }
        winner = cmp::max(winner, dist);
    }
    winner
}

fn part2(cont: &str) -> u32 {
    let re =
        Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)")
            .unwrap();
    //let mut winner: u32 = 0;
    let mut riders = vec![];
    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        riders.push((
            //caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        ));
    }

    let mut scores: Vec<u32> = vec![0; riders.len()];
    for t in 1..=TOTAL_TIME {
        let mut cur_scores = vec![0; riders.len()];
        for (i, r) in riders.iter().enumerate() {
            cur_scores[i] = calc_dist(t, r.0, r.1, r.2);
        }
        let max = cur_scores.iter().max().unwrap();
        for (i, v) in cur_scores.iter().enumerate() {
            if v == max {
                scores[i] += 1;
            }
        }
    }
    *scores.iter().max().unwrap()
}

fn calc_dist(time: u32, speed: u32, speed_time: u32, rest_time: u32) -> u32 {
    let tt = speed_time + rest_time;
    let mut dist = (time / tt) * (speed * speed_time);
    let r = time % tt;
    if r <= speed_time {
        dist += r * speed;
    } else {
        dist += speed_time * speed
    }
    dist
}
