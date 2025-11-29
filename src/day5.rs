use std::path::Path;
use crate::intcomp::{IntComp,get_user_input};
use std::sync::mpsc;

pub fn run() -> (isize,isize) {
    (pt1(),pt2())
}

fn pt1() -> isize {
    let path = Path::new(crate::BASE_DIR).join("real").join("day5.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let (tx,rx) = mpsc::channel();
    let mut intcomp = IntComp::from_string(program_str,tx, Box::new(get_user_input));
    intcomp.run_program();
    drop(intcomp);
    let output = rx.iter().collect::<Vec<String>>();
    output.last().unwrap().parse::<isize>().unwrap()
}

fn pt2() -> isize {
        let path = Path::new(crate::BASE_DIR).join("real").join("day5.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let (tx,rx) = mpsc::channel();
    let mut intcomp = IntComp::from_string(program_str,tx,Box::new(get_user_input));
    intcomp.run_program();
    drop(intcomp);
    let output = rx.iter().collect::<Vec<String>>();
    output.last().unwrap().parse::<isize>().unwrap()
}