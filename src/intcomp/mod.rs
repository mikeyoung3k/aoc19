// Store memory as Vec<&str>
// Look at where instr pntr goes and get the opcode block
// Assess len (assign) and split into param modes and opcode
// Return - and get the values of the parameters
// Based on op code, create Instruction type with parameters
// Run Instruction and execute effect 
// Move pointer based on len from beginning

use std::fmt::Error;


enum Instruction {
    Add(Vec<isize>),
    Multiply(Vec<isize>),
    Store(isize),
    Load(isize),
    Terminate {},
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
        if self.memory.len() >= addr {
            return Err("Invalid store memory address".to_string())
        }
        self.memory[addr] = value;
        Ok(())
    }

    fn parse_instruction(&self) -> Result<Instruction, String> {
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

    
}
