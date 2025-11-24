use std::path::Path;
use crate::intcomp::IntComp;

pub fn run() -> (isize,isize) {
    (pt1(),pt2())
}

fn pt1() -> isize {
    let path = Path::new(crate::BASE_DIR).join("real").join("day5.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let mut intcomp = IntComp::from_string(program_str);
    intcomp.run_program();
    println!("{:?}", intcomp.output_store);
    intcomp.output_store[intcomp.output_store.len() - 1]
}

fn pt2() -> isize {
        let path = Path::new(crate::BASE_DIR).join("real").join("day5.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let mut intcomp = IntComp::from_string(program_str);
    intcomp.run_program();
    println!("{:?}", intcomp.output_store);
    intcomp.output_store[intcomp.output_store.len() - 1]
}