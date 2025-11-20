// Store memory as Vec<&str>
// Look at where instr pntr goes and get the opcode block
// Assess len (assign) and split into param modes and opcode
// Return - and get the values of the parameters
// Based on op code, create Instruction type with parameters
// Run Instruction and execute effect 
// Move pointer based on len from beginning

use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Add(Parameters),
    Multiply(Parameters),
    Store(Parameters),
    Load(usize),
    Terminate {},
}


#[derive(Debug, PartialEq, Eq)]
struct Parameters {
    inputs: Vec<isize>,
    store_location: usize,
}

struct IntComp {
    memory: Vec<String>, // vec!["01002","3","4","5"]
    instr_pntr: usize,
}

impl IntComp {
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
                let param_a = self.parse_param(a_mode)?;
                Ok(Instruction::Load(param_a as usize))
            }, // Load
            99 => Ok(Instruction::Terminate {}), // Terminate
            _ => Err("Invalid opcode".to_string())?,
        }
    }

    fn run_instruction(&mut self) {
        todo!();
        // Loads parameter values manually
        // Call run for the instruction
        // Store value manually
        // This fn must keep track of output location
        // This fn must parse parameter types and get final values
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

    #[test]
    fn test_instruction_parsing() {
        let mut intcomp = new_testcomp();
        assert_eq!(intcomp.instr_pntr,0);
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        assert_eq!(intcomp.instr_pntr,4);
        let expect_params = Parameters { inputs: vec![33, 3], store_location: 4 };
        let expect_instr = Instruction::Multiply(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }
    
    fn new_testcomp() -> IntComp {
        let test_mem = vec!["1002", "4","3","4","33"].iter().map(|s| s.to_string()).collect();
        IntComp { memory: test_mem, instr_pntr: 0 }
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
