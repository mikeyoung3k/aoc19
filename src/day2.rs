use std::path::Path;


pub fn run() -> (isize,isize) {
    let path = Path::new(crate::BASE_DIR).join("real").join("day2.txt");
    let program_str = std::fs::read_to_string(path).expect("Failed to read file");
    let p1 = pt1(&program_str);
    let p2 = pt2(&program_str);
    (p1,p2)
}

fn pt1(prog_str: &str) -> isize {
    let mut prog_vec = parse_str(prog_str);
    prog_vec[1] = 12;
    prog_vec[2] = 2;
    run_program(&mut prog_vec)

}

fn pt2(prog_str: &str) -> isize {
    let prog_vec = parse_str(prog_str);
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut prog_copy = prog_vec.clone();
                prog_copy[1] = noun;
                prog_copy[2] = verb;
                if run_program(&mut prog_copy) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        panic!("No solution found");
}

pub fn parse_str(prog_str: &str) -> Vec<isize> {
    prog_str.split(',')
    .map(|s| s.parse().expect("Failed to parse instruction"))
    .collect()
}

pub fn run_program(prog: &mut Vec<isize>) -> isize {
    let mut instr_ptr = 0;
    let mut acc = Action::Continue;
    while acc == Action::Continue {
        acc = run_instruction(prog, instr_ptr);
        instr_ptr += 4;
    }
    prog[0]
}

fn run_instruction(prog: &mut Vec<isize>, instr_pnt: usize) -> Action {
    let instr = prog.get(instr_pnt).copied();
    let a_idx = prog.get(instr_pnt+1).copied();
    let b_idx = prog.get(instr_pnt+2).copied();
    let store = prog.get(instr_pnt+3).copied();
    match instr {
        Some(1) => {
            // Add
            prog[store.unwrap() as usize] =
                prog[a_idx.unwrap() as usize] + prog[b_idx.unwrap() as usize];
        },
        Some(2) => {
            // Multiply
            prog[store.unwrap() as usize] =
                prog[a_idx.unwrap() as usize] * prog[b_idx.unwrap() as usize];
        },
        Some(99) => {
            // Terminate
            return Action::Terminate;
        }
        Some(_) => return Action::InvalidCode,
        None => return Action::PrematureEnd,
    }
    Action::Continue
}

#[derive(Debug, PartialEq)]
enum Action {
    Continue,
    Terminate,
    InvalidCode,
    PrematureEnd
}


#[cfg(test)]
mod test {
    use super::*;
    const TEST_STR: & str = "1,9,10,3,2,3,11,0,99,30,40,50";


    #[test]
    fn test_instruction_parsing() {
        let prog_vec: Vec<isize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        let mut input_vec = parse_str(TEST_STR);
        let r = run_program(&mut input_vec);
        assert_eq!(input_vec, prog_vec);
        assert_eq!(r, 3500);
    }

    #[test]
    fn test_single_instruction() {
        let mut input_vec = parse_str(TEST_STR);
        let one_cycle_vec = vec![1, 9, 10, 70,2,3,11,0,99,30,40,50 ];
        let one_cycle_res = run_instruction(&mut input_vec, 0);
        assert_eq!(one_cycle_res, Action::Continue);
        assert_eq!(one_cycle_vec, input_vec);
    }

}