// Store memory as Vec<&str>
// Look at where instr pntr goes and get the opcode block
// Assess len (assign) and split into param modes and opcode
// Return - and get the values of the parameters
// Based on op code, create Instruction type with parameters
// Run Instruction and execute effect 
// Move pointer based on len from beginning

use std::fmt::Error;

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

    fn parse_next_instruction(&self) -> Result<Instruction, String> {
        todo!();
        // Load memory at instruction pointer
        // Check num chars -> Value to move instruction pointer by
        // Check parameter modes and get values
        // Create Instruction with parameter values
        // Return
    }


    fn run_instruction(&mut self) {
        todo!();
        // Loads parameter values manually
        // Call run for the instruction
        // Store value manually
        // This fn must keep track of output location
        // This fn must parse parameter types and get final values
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_parsing() {
        let intcomp = new_testcomp();
        let next_instr = intcomp.parse_next_instruction();
        assert!(next_instr.is_ok());
        let expect_params = Parameters { inputs: vec![9, 30], store_location: 3 };
        let expect_instr = Instruction::Multiply(expect_params);
        assert_eq!(next_instr.unwrap(), expect_instr);
    }
    
    fn new_testcomp() -> IntComp {
        // 0 is position mode
        // 1 is immediate mode
        // 01002,9,10,3
        // 1st Param is immediate mode - value is 9
        // 2nd Param is position mode - value is the 10th item in memory - 30
        // 3rd Param is immediate mode - the output is written to position 3
        let test_mem = vec!["01002", "9", "10", "3", "2", "3", "11", "0", "99", "30"].iter().map(|s| s.to_string()).collect();
        IntComp { memory: test_mem, instr_pntr: 0 }
    }

    #[test]
    fn test_load_mem() {
        let comp = new_testcomp();
        assert_eq!(comp.load_mem(0), Ok("01002"));
        assert!(comp.load_mem(200000).is_err());
    }

    #[test]
    fn test_store_mem() {
        let mut comp = new_testcomp();
        assert_eq!(comp.memory[0], "01002".to_string());
        assert!(comp.store_mem(0, "99999".to_string()).is_ok());
        assert_eq!(comp.memory[0], "99999".to_string());
        assert!(comp.store_mem(2000000,"88888".to_string()).is_err());
    }
    
}
