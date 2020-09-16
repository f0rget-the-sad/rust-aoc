use std::fs;

use serde_json::Value;

fn main() {
    let filename = "src/2015/day12/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 191164);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 87842);
}

fn part1(cont: &str) -> i64 {
    let js: Value = serde_json::from_str(cont).unwrap();
    calc_sum(&js)
}

fn part2(cont: &str) -> i64 {
    let js: Value = serde_json::from_str(cont).unwrap();
    calc_sum2(&js)
}

fn calc_sum(jobj: &Value) -> i64 {
    let mut sum: i64 = 0;
    match jobj {
        Value::Object(obj) => {
            for (_, v) in obj {
                sum += calc_sum(&v);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                sum += calc_sum(&v);
            }
        }
        Value::Number(n) => {
            //dbg!(n);
            sum += n.as_i64().unwrap();
        }
        _ => return sum,
    };
    sum
}

fn calc_sum2(jobj: &Value) -> i64 {
    let mut sum: i64 = 0;
    match jobj {
        Value::Object(obj) => {
            for (_, v) in obj {
                if v.is_string() {
                    if v.as_str().unwrap() == "red" {
                        return sum;
                    }
                }
            }
            for (_, v) in obj {
                sum += calc_sum2(&v);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                sum += calc_sum2(&v);
            }
        }
        Value::Number(n) => {
            sum += n.as_i64().unwrap();
        }
        _ => return sum,
    };
    sum
}
