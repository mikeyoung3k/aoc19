use std::path::Path;

pub fn run() -> (isize,isize) {
    (pt1(),pt2())
}

fn pt1() -> isize {
    let path = Path::new(crate::BASE_DIR).join("real").join("day2.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let mut intcomp = IntComp::from_string(program_str);
    
}

fn pt2() -> isize {
    0
}