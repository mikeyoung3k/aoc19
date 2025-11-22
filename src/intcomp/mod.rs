use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Add(Parameters),
    Multiply(Parameters),
    Store(Parameters),
    Load(usize),
    Terminate,
}


#[derive(Debug, PartialEq, Eq)]
struct Parameters {
    inputs: Vec<isize>,
    store_location: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct IntComp {
    memory: Vec<String>, // vec!["01002","3","4","5"]
    instr_pntr: usize,
    output_store: Vec<isize>,
}

impl IntComp {
    fn from_string(s: String) -> IntComp {
        let memory = s.split(",").map(|s| s.to_string()).collect();
        IntComp {
            memory,
            instr_pntr: 0,
            output_store: Vec::new(),
        }
    }
    fn run_program(&mut self) {
        while !self.run_instruction(){

        };
    }
    fn load_mem(&self, addr: usize) -> Result<&str,String> {
        self.memory.get(addr).map(|s| s.as_str()).ok_or("Invalid load memory address".to_owned())
    }

    fn store_mem(&mut self, addr: usize, value: String) -> Result<(), String> {
        if addr >= self.memory.len() {
            return Err("Invalid store memory address".to_string())
        }
        self.memory[addr] = value;
        Ok(())
    }

    fn parse_next_instruction(&mut self) -> Result<Instruction, Box<dyn Error>> {
        // Load memory at instruction pointer
        // Check num chars -> Value to move instruction pointer by
        // Check parameter modes and get values
        // Create Instruction with parameter values
        // Return
        let instr = self.load_mem(self.instr_pntr)?.parse::<usize>().expect("Failed to parse instruction");
        self.instr_pntr += 1;

        // 1002
        let opcode = instr % 100;
        let a_mode = (instr/100) % 10;
        let b_mode = (instr/1000) % 10;
        let c_mode = (instr/10000) % 10;
        // 01002,9,10,3

        match opcode {
            1 => {
                let param_a = self.parse_param(a_mode)?;
                let param_b = self.parse_param(b_mode)?;
                let param_c = self.parse_param(1)?;
                Ok(Instruction::Add(Parameters { inputs: vec![param_a, param_b], store_location: param_c as usize }))
            }, // Add
            2 => { // Multiply
                let param_a = self.parse_param(a_mode)?;
                let param_b = self.parse_param(b_mode)?;
                let param_c = self.parse_param(1)?;
                Ok(Instruction::Multiply(Parameters { inputs: vec![param_a, param_b], store_location: param_c as usize }))
            },
            3 => {
                let param_a = self.parse_param(a_mode)?;
                let param_b = self.parse_param(1)?;
                Ok(Instruction::Store(Parameters { inputs: vec![param_a], store_location: param_b as usize }))
            }, // Store
            4 => {
                let param_a = self.parse_param(1)?;
                Ok(Instruction::Load(param_a as usize))
            }, // Load
            99 => Ok(Instruction::Terminate), // Terminate
            _ => Err("Invalid opcode".to_string())?,
        }
    }

    fn run_instruction(&mut self) -> bool {
        let next = self.parse_next_instruction().expect("Failed to parse next instruction");
        match next {
            Instruction::Add(params) => {
                self.store_mem(params.store_location, params.inputs.iter().sum::<isize>().to_string()).expect("Failed to store add");
            },
            Instruction::Multiply(Parameters{inputs,store_location}) => {
                self.store_mem(store_location, inputs.iter().fold(1,|acc,x| acc * x).to_string()).expect("Failed to store multiply");
            },
            Instruction::Store(params) => {
                self.store_mem(params.store_location, params.inputs[0].to_string()).expect("Failed to store value");
            },
            Instruction::Load(loc) => {
                self.output_store.push(self.load_mem(loc).expect("Failed to load value").parse::<isize>().expect("Failed to parse memory as isize"));
            },
            Instruction::Terminate => {return true},
        }
        false
    }

    fn parse_param(&mut self, mode: usize) -> Result<isize, Box<dyn Error>> {
        let r = if mode == 0 { // Positional
            let param_location = self.load_mem(self.instr_pntr)?.parse::<usize>().expect("Failed to parse parameter location");
            self.load_mem(param_location)?.parse::<isize>()?
        } else {
            self.load_mem(self.instr_pntr)?.parse::<isize>()?
        };
        self.instr_pntr += 1;
        Ok(r)
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn new_testcomp() -> IntComp {
        let test_mem = vec!["1002", "4","3","4","33"].iter().map(|s| s.to_string()).collect();
        IntComp { memory: test_mem, instr_pntr: 0, output_store:vec![] }
    }

    #[test]
    fn test_new_intcomp() {
        let test_string = "1002,4,3,4,33".to_string();
        let intcomp = IntComp::from_string(test_string);
        let expect = new_testcomp();
        assert_eq!(intcomp, expect);
    }

    #[test]
    fn test_instruction_parsing_mult() {
        let mut intcomp = new_testcomp();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,4);
        let expect_params = Parameters { inputs: vec![33, 3], store_location: 4 };
        let expect_instr = Instruction::Multiply(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }

    #[test]
    fn test_instruction_parsing_add() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1001".to_string();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,4);
        let expect_params = Parameters { inputs: vec![33, 3], store_location: 4 };
        let expect_instr = Instruction::Add(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }
    
    #[test]
    fn test_instruction_parsing_store() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1003".to_string();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,3);
        let expect_params = Parameters { inputs: vec![33], store_location: 3 };
        let expect_instr = Instruction::Store(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }
    
    #[test]
    fn test_instruction_parsing_load() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1004".to_string();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,2);
        let expect_params = 4;
        let expect_instr = Instruction::Load(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }
    
    #[test]
    fn test_instruction_parsing_terminate() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1099".to_string();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,1);
        let expect_instr = Instruction::Terminate;
        assert_eq!(next_instr.unwrap(), expect_instr);
    }

    #[test]
    fn test_run_add() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1001".to_string();
        assert_eq!(intcomp.memory[4],"33".to_string());
        let out = intcomp.run_instruction();
        assert_eq!(intcomp.memory[4],"36".to_string());
        assert!(!out);
    }

    #[test]
    fn test_run_mult() {
        let mut intcomp = new_testcomp();
        assert_eq!(intcomp.memory[4],"33".to_string());
        let out = intcomp.run_instruction();
        assert_eq!(intcomp.memory[4],"99".to_string());
        assert!(!out);
    }

    #[test]
    fn test_run_store() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1003".to_string();
        assert_eq!(intcomp.memory[3],"4".to_string());
        assert!(!intcomp.run_instruction());
        assert_eq!(intcomp.memory[3],"33".to_string());
    }

    #[test]
    fn test_run_load() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1004".to_string();
        assert!(!intcomp.run_instruction());
        assert!(intcomp.output_store[0] == 33);
    }
    
    #[test]
    fn test_run_terminate() {
        let mut intcomp = new_testcomp();
        intcomp.memory[0] = "1099".to_string();
        assert!(intcomp.run_instruction());
    }

    #[test]
    fn test_load_mem() {
        let comp = new_testcomp();
        assert_eq!(comp.load_mem(0), Ok("1002"));
        assert!(comp.load_mem(200000).is_err());
    }

    #[test]
    fn test_store_mem() {
        let mut comp = new_testcomp();
        assert_eq!(comp.memory[0], "1002".to_string());
        assert!(comp.store_mem(0, "99999".to_string()).is_ok());
        assert_eq!(comp.memory[0], "99999".to_string());
        assert!(comp.store_mem(2000000,"88888".to_string()).is_err());
    }
    
}
