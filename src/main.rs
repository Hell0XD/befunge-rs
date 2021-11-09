use std::io::{stdin, Read};
fn increase_direction(x: &mut usize, y: &mut usize, dir: u8) {
    if dir == 0 {
        *x += 1;
    } else if dir == 1 {
        *y += 1;
    } else if dir == 2 {
        *x -= 1;
    } else {
        *y -= 1;
    }
}
fn main() {
    let file = std::fs::read_to_string(std::env::args().skip(1).next().unwrap()).unwrap();
    let program = file.split_terminator("\n").map(|l| l.chars().collect());
    let mut program: Vec<Vec<char>> = program.collect();
    let (mut x, mut y, mut direction, mut stack): (_, _, u8, Vec<i32>) = (0, 0, 0, Vec::new()); // 0 >, 1 v, 2 <, 3 ^
    loop {
        match program[y][x] {
            num if num.is_numeric() => stack.push(num.to_digit(10).unwrap() as i32),
            '+' => {
                let result = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(result);
            }
            '-' => {
                let result = stack.pop().map(|a| stack.pop().unwrap() - a).unwrap();
                stack.push(result);
            }
            '*' => {
                let result = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(result);
            }
            '/' => {
                let result = stack.pop().map(|a| stack.pop().unwrap() / a).unwrap();
                stack.push(result);
            }
            '%' => {
                let result = stack.pop().map(|a| stack.pop().unwrap() % a).unwrap();
                stack.push(result);
            }
            '!' => stack
                .last_mut()
                .map(|x| if *x == 0 { *x = 1 } else { *x = 0 })
                .unwrap(),
            '`' => {
                let bool = stack.pop().map(|a| stack.pop().unwrap() > a).unwrap();
                stack.push(bool as i32);
            }
            '>' => direction = 0,
            'v' => direction = 1,
            '<' => direction = 2,
            '^' => direction = 3,
            '?' => {
                direction = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|dur| (dur.as_micros() as u8).to_string().chars().last().unwrap())
                    .map(|char| (char.to_digit(10).unwrap() / 3) as u8)
                    .unwrap()
            }
            '_' => direction = if stack.pop().unwrap() == 0 { 0 } else { 2 },
            '|' => direction = if stack.pop().unwrap() == 0 { 1 } else { 3 },
            '"' => {
                increase_direction(&mut x, &mut y, direction);
                while program[y][x] != '"' {
                    stack.push(program[y][x] as i32);
                    increase_direction(&mut x, &mut y, direction);
                }
            }
            ':' => stack.push(*stack.last().unwrap()),
            '\\' => {
                let len = stack.len();
                stack.swap(len - 2, len - 1);
            }
            '$' => stack.pop().map(|_| ()).unwrap(),
            '.' => print!("{} ", stack.pop().unwrap()),
            ',' => print!("{}", stack.pop().unwrap() as u8 as char),
            '#' => increase_direction(&mut x, &mut y, direction),
            'p' => {
                program[stack.pop().unwrap() as usize][stack.pop().unwrap() as usize] =
                    stack.pop().unwrap() as u8 as char
            }
            'g' => {
                let result = program[stack.pop().unwrap() as usize][stack.pop().unwrap() as usize];
                stack.push(result as i32);
            }
            '&' => stack.push(
                stdin()
                    .bytes()
                    .fold(String::new(), |c, h| c + &(h.unwrap() as char).to_string())
                    .parse()
                    .unwrap(),
            ),
            '~' => stack.push(stdin().bytes().next().unwrap().unwrap() as i32),
            '@' => break,
            ' ' => (),
            _ => panic!("Unexpected character"),
        }
        increase_direction(&mut x, &mut y, direction);
    }
}
