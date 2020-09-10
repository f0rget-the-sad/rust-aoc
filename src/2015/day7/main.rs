use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "src/2015/day7/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    /* Example
    let cont = "123 -> x\n\
        456 -> y\n\
        x AND y -> d\n\
        x OR y -> e\n\
        x LSHIFT 2 -> f\n\
        y RSHIFT 2 -> g\n\
        NOT x -> h\n\
        NOT y -> i";
    */
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 46065);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 14134);
}

fn part1(cont: &str) -> u16 {
    //Each wire can only get a signal from one source
    // so make source key in hash table
    let mut wires = HashMap::new();
    for line in cont.lines() {
        let mut parts = line.split(" -> ");
        let gates = parts.next().unwrap();
        let wire = parts.next().unwrap();
        wires.insert(wire, gates);
    }
    let mut parsed_wires = HashMap::new();
    parse_signal("a", &wires, &mut parsed_wires)
}

fn part2(cont: &str) -> u16 {
    //Each wire can only get a signal from one source
    // so make source key in hash table
    let mut wires = HashMap::new();
    for line in cont.lines() {
        let mut parts = line.split(" -> ");
        let gates = parts.next().unwrap();
        let wire = parts.next().unwrap();
        wires.insert(wire, gates);
    }
    let mut parsed_wires = HashMap::new();
    let a_value = parse_signal("a", &wires, &mut parsed_wires);
    parsed_wires.clear();
    parsed_wires.insert("b", a_value);
    parse_signal("a", &wires, &mut parsed_wires)
}

fn parse_signal<'a>(
    wire: &'a str,
    wires: &HashMap<&str, &'a str>,
    parsed_wires: &mut HashMap<&'a str, u16>,
) -> u16 {
    // wire or number
    match wire.parse::<u16>() {
        Ok(n) => return n,
        Err(_) => {}
    };
    // check if wire was already parsed
    match parsed_wires.get(wire) {
        Some(sig_value) => return *sig_value,
        None => {}
    }
    // parsing...
    let parts: Vec<&str> = wires[wire].split_whitespace().collect();
    return match parts.len() {
        // 1 - number or wire
        1 => match parts[0].parse::<u16>() {
            Ok(n) => {
                parsed_wires.insert(wire, n);
                n
            }
            Err(_) => parse_signal(parts[0], wires, parsed_wires),
        },
        // 2 - NOT wire
        2 => {
            let n = !parse_signal(parts[1], wires, parsed_wires);
            parsed_wires.insert(wire, n);
            n
        }
        // 3 - (number or wire) cmd (number or wire)
        3 => {
            let a1 = parse_signal(parts[0], wires, parsed_wires);
            let a2 = parse_signal(parts[2], wires, parsed_wires);
            let n = match parts[1] {
                "AND" => a1 & a2,
                "OR" => a1 | a2,
                "LSHIFT" => a1 << a2,
                "RSHIFT" => a1 >> a2,
                _ => panic!("Unknow op"),
            };
            parsed_wires.insert(wire, n);
            n
        }
        _ => panic!("Unexpected N of args"),
    };
}
