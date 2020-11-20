use regex::Regex;
use std::cmp;
use std::fs;

fn main() {
    let filename = "src/2015/day15/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 21367368);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 1766400);
}

fn part1(cont: &str) -> u32 {
    let re = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();
    let mut ingredients = vec![];
    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        let fields = vec![
            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
            // ignore calories for now
            //caps.get(6).unwrap().as_str().parse::<i32>().unwrap(),
        ];
        ingredients.push(fields);
    }
    let mut best = 0;
    for destrib in integer_part_with_k_size(100, ingredients.len()) {
        best = cmp::max(calc_propert(&ingredients, &destrib), best);
    }
    best
}

fn part2(cont: &str) -> u32 {
    let re = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();
    let mut ingredients = vec![];
    let mut calories = vec![];
    for l in cont.lines() {
        let caps = re.captures(l).unwrap();
        let fields = vec![
            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
        ];
        ingredients.push(fields);
        calories.push(caps.get(6).unwrap().as_str().parse::<u16>().unwrap());
    }
    let mut best = 0;
    for destrib in integer_part_with_k_size(100, ingredients.len()) {
        if calc_calories(&calories, &destrib) != 500 {
            continue;
        }
        best = cmp::max(calc_propert(&ingredients, &destrib), best);
    }
    best
}

fn calc_propert(ingredients: &Vec<Vec<i32>>, distribution: &Vec<u16>) -> u32 {
    let mut score = 1;
    let param_len = ingredients[0].len();
    for j in 0..param_len {
        let mut sum : i32= 0;
        for i in 0..ingredients.len() {
            sum += ingredients[i][j] * distribution[i] as i32;
        }
        if sum < 0 {
            return 0;
        }
        score *= sum as u32;
    }
    score
}

fn calc_calories(calories: &Vec<u16>, distribution: &Vec<u16>)-> u16
{
    let mut sum = 0;
    for i in 0..calories.len() {
        sum += calories[i]*distribution[i];
    }
    sum
}


/* TODO:
 * - use iterators
 * - cash values
 * https://stackoverflow.com/a/18503368
 * */
fn integer_part_with_k_size(number: u16, size: usize) -> Vec<Vec<u16>> {
    if size == 1 {
        return vec![vec![number]];
    }
    let mut ret = vec![];
    for i in 0..number + 1 {
        for mut j in integer_part_with_k_size(number - i, size - 1) {
            j.push(i);
            ret.push(j);
        }
    }
    ret
}
