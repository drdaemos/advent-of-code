use std::{
    collections::HashMap,
    io::{stdout, Write},
};

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn sub_pipes(input: char) -> char {
    return match input {
        '|' => '║',
        '-' => '═',
        'L' => '╚',
        'F' => '╔',
        'J' => '╝',
        '7' => '╗',
        '.' => '░',
        'S' => '╳',
        _ => todo!("unknown char"),
    };
}

pub fn render_map(graph: &HashMap<&(usize, usize), char>) {
    let mut max_line = 0;

    for y in 0..140 {
        for x in 0..140 {
            if graph.contains_key(&(x, y)) {
                let ch = graph.get(&(x, y)).unwrap();
                print!("{}", ch);
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!();
}
