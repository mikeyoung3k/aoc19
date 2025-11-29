use itertools::Itertools;
use crate::intcomp::IntComp;
use std::fs::read_to_string;
use std::path::Path;
use crate::BASE_DIR;
use std::sync::mpsc;
use std::thread;

pub fn run() -> (isize, isize) {
    let path = Path::new(BASE_DIR).join("real").join("day7.txt");
    let input_string = read_to_string(path).expect("Error reading file");
    (pt1(&input_string),pt2(&input_string))
}

fn pt1(data: &String) -> isize {
    let mut max_output = isize::MIN;
    for mut combo in (0..5).map(|x| x.to_string()).permutations(5) {
        // Create Amplifier 1, with first input of phase setting, second input of 0
        // Create in turn, Amplifiers 2-5 with first input of phase setting and second input of previous amp out
        let mut output = "0".to_string();
        for _ in 1..=5 {
            let mut inputs = vec![output, combo.pop().unwrap()]; // [0, phase_setting]
            let (tx,rx) = mpsc::channel();
            let mut amp = IntComp::from_string(data.to_owned(),tx, Box::new(move || inputs.pop().unwrap()));
            amp.run_program();
            drop(amp);
            let output_store = rx.iter().collect::<Vec<String>>();
            output = output_store.last().unwrap().to_owned();
        }
        if output.parse::<isize>().unwrap() > max_output {
            max_output = output.parse::<isize>().unwrap();
        }
    }
    max_output
}

fn pt2(data: &String) -> isize {
    let mut max_output = isize::MIN;
    for mut combo in (5..=9).map(|x| x.to_string()).permutations(5) {
        // Create Amplifier 1, with first input of phase setting, second input of 0
        // Create in turn, Amplifiers 2-5 with first input of phase setting and second input of previous amp out
        let output = single_iteration(data, combo);
        if output.parse::<isize>().unwrap() > max_output {
            max_output = output.parse::<isize>().unwrap();
        }
    }
    max_output
}

fn single_iteration(data: &String, mut phase_settings: Vec<String>) -> String {
    let (tx0,mut rx0) = mpsc::channel::<String>();
    // tx0 is the output of the last amp
    // rx0 is the input of the first amp
    let phase = phase_settings.pop().unwrap();
    tx0.send(phase).expect("Channel closed unexpectedly sending phase information");
    tx0.send("0".to_string()).expect("Channel closed unexpectedly sending initial input");
    
    let mut amps = vec![];
    for _ in 1..=5 {
        let (tx1, rx1) = mpsc::channel::<String>();
        if let Some(phase) = phase_settings.pop() {
            tx1.send(phase).expect("Channel closed unexpectedly sending phase information");
        }
        let input_f = move || {
            rx0.recv().expect("Channel closed unexpectedly receiving input")
        };
        rx0 = rx1;

        amps.push(IntComp::from_string(data.to_owned(), tx1, Box::new(input_f)));
    }

    let mut last = amps.pop().unwrap();
    last.output_store = tx0;
    amps.push(last);

    let mut handles = vec![];
    for i in 1..=5 {
        let mut amp = amps.pop().unwrap();  
        let handle = thread::spawn(move || {
            amp.run_program();
            if i == 5 {
                Some((amp.input)())
            } else {
                None
            }
        });
        handles.push(handle);
    }

    let last = handles.pop().unwrap();
    last.join().expect("Thread panicked").unwrap()
    // "0".to_string()
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pt1() {
        let test = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0].iter().map(|x| x.to_string()).join(",");
        assert_eq!(pt1(&test), 43210);

        let test = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0].iter().map(|x| x.to_string()).join(",");
        assert_eq!(pt1(&test), 54321);

        let test = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0].iter().map(|x| x.to_string()).join(",");
        assert_eq!(pt1(&test), 65210);
    }

    #[test]
    fn test_pt2() {
        let test = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5].iter().map(|x| x.to_string()).join(",");
        assert_eq!(pt2(&test), 139629729);
        
        let test = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10].iter().map(|x| x.to_string()).join(",");
        assert_eq!(pt2(&test), 18216);

    }
}