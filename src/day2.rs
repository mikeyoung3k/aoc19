use std::path::Path;

use crate::intcomp;

pub fn run() -> (isize,isize) {
    let path = Path::new(crate::BASE_DIR).join("real").join("day2.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let p1 = pt1(&program_str);
    let p2 = pt2(&program_str);
    (p1,p2)
}

fn pt1(prog_str: &str) -> isize {
    let mut prog_vec = intcomp::parse_str(prog_str);
    prog_vec[1] = 12;
    prog_vec[2] = 2;
    intcomp::run_program(&mut prog_vec)

}

fn pt2(prog_str: &str) -> isize {
    let mut prog_vec = intcomp::parse_str(prog_str);
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut prog_copy = prog_vec.clone();
                prog_copy[1] = noun;
                prog_copy[2] = verb;
                if intcomp::run_program(&mut prog_copy) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        panic!("No solution found");
}