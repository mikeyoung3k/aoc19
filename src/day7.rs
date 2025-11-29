pub fn run() -> (usize, usize) {
    (0,0)
}

fn pt1() -> usize {
    let inputs = vec![0,3];
    let inputter = Box::new(move || inputs.pop().unwrap());

    let mut computer = IntComputer::new(vec["10012".to_string()],inputter);
    
}