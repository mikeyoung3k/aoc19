// Store memory as Vec<&str>
// Look at where instr pntr goes and get the opcode block
// Assess len (assign) and split into param modes and opcode
// Return - and get the values of the parameters
// Based on op code, create Instruction type with parameters
// Run Instruction and execute effect 
// Move pointer based on len from beginning


enum Instruction {
    Add(Vec<isize>),
    Multiply(Vec<isize>),
    Store(isize),
    Load(isize),
    Terminate {},
}

struct IntComp {
    memory: Vec<&str>, // vec!["01002","3","4","5"]
    instr_pntr: usize,
}

impl IntComp {
    fn parse_instruction(&self,instruction: &[isize]) -> Instruction {
        // Instruction format:
        // ABCDE
        // A, B, C: Parameter modes for parameters 3, 2, and 1 respectively
        // DE: OpCode
        let opcode = instruction[0] % 100;
        // let a_mode = (instruction[0] / 100) % 10;
        // let b_mode = (instruction[0] / 1000) % 10;
        // let c_mode = (instruction[0] / 10000) % 10;
        // let a = if a_mode == 0 {
        //     self.memory[instruction[1] as usize]
        // } else {
        //     instruction[1]
        // };
        // let b = if b_mode == 0 {
        //     self.memory[instruction[2] as usize]
        // } else {
        //     instruction[2]
        // };
        // let c = if c_mode == 0 {
        //     self.memory[instruction[3] as usize]
        // } else {
        //     instruction[3]
        // };
        match opcode {
            1 => {
                let a = if a_mode == 0 {
                    self.memory[instruction[1] as usize]
                } else {
                    instruction[1]
                };
                let b = if b_mode == 0 {
                    self.memory[instruction[2] as usize]
                } else {
                    instruction[2]
                };
                let c = if c_mode == 0 {
                    self.memory[instruction[3] as usize]
                } else {
                    instruction[3]
                };
                Instruction::Add(vec![a, b, c])
            }
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

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let instr = vec![1002,4,3,4];
        let memory = vec![1, 2, 3, 4];
        let expect = Some(12);

        let comp = IntComp{memory: memory, instr_pntr: 0};
        let result = comp.parse_instruction(&instr);

        assert_eq!(expect,result.run());
    }
}
