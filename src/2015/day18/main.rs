use std::fs;

fn main() {
    let filename = "src/2015/day18/input";
    let cont = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let res1 = part1(&cont);
    println!("part1 :\n{}", res1);
    assert_eq!(res1, 768);

    let res2 = part2(&cont);
    println!("part2 :\n{}", res2);
    assert_eq!(res2, 781);
}

const SIZE: usize = 100;
const MAX_IDX: usize = SIZE - 1;
const ON: char = '#';
const OFF: char = '.';

fn count_neighbors(field: &[[char; SIZE]; SIZE], i: usize, j: usize) -> u8 {
    let mut n = 0;
    //[   ][i-1][   ] 1 row
    //[j-1][i;j][j+1] 2 row
    //[   ][i+i][   ] 3 row
    // top 3 cells (1 row)
    if i != 0 {
        if j != 0 {
            if field[i - 1][j - 1] == ON {
                n += 1;
            }
        }
        if field[i - 1][j] == ON {
            n += 1;
        }
        if j != MAX_IDX {
            if field[i - 1][j + 1] == ON {
                n += 1;
            }
        }
    }
    // row 2
    // current cell[i][j] is not counted
    if j != 0 {
            if field[i][j - 1] == ON {
                n += 1;
            }
    }
    if j != MAX_IDX {
            if field[i][j + 1] == ON {
                n += 1;
            }
    }
    //row 3
    if i != MAX_IDX {
        if j != 0 {
            if field[i + 1][j - 1] == ON {
                n += 1;
            }
        }
        if field[i + 1][j] == ON {
            n += 1;
        }
        if j != MAX_IDX {
            if field[i + 1][j + 1] == ON {
                n += 1;
            }
        }
    }
    n
}

fn cell_next_state(field: &[[char; SIZE]; SIZE], i: usize, j: usize) -> char {
    let neighbors = count_neighbors(field, i, j);
    if (field[i][j] == ON && (neighbors == 2 || neighbors == 3)) ||
        (field[i][j] == OFF && neighbors == 3 ) {
        return ON;
    }

    return OFF;
}
fn set_corners(field: &mut[[char; SIZE]; SIZE]) {
    field[0][0] = ON;
    field[0][MAX_IDX] = ON;
    field[MAX_IDX][0] = ON;
    field[MAX_IDX][MAX_IDX] = ON;
}

fn next(current_field: &mut[[char; SIZE]; SIZE]) {
    let mut next_field = [[0 as char; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            next_field[i][j] = cell_next_state(current_field, i, j);
        }
    }
    *current_field = next_field;
}

fn fsum(field: &[[char; SIZE]; SIZE]) -> u32 {

    let mut sum = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if field[i][j] == ON {
                sum += 1
            }
        }
    }
    sum
}

fn part1(cont: &str) -> u32 {
    let mut field = [[0 as char; SIZE]; SIZE];
    for (i, l) in cont.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            field[i][j] = c;
        }
    }
    for _ in 0..SIZE {
        next(&mut field);
    }
    fsum(&field)
}

fn part2(cont: &str) -> u32 {
    let mut field = [[0 as char; SIZE]; SIZE];
    for (i, l) in cont.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            field[i][j] = c;
        }
    }
    set_corners(&mut field);
    for _ in 0..SIZE {
        next(&mut field);
        set_corners(&mut field);
    }
    fsum(&field)
}
