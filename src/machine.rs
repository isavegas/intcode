use crate::deconstruct_opcode;
use crate::ErrorStatus;
use crate::ExecutionStatus;
use crate::Memory;
use crate::Num;
use crate::ParamMode;

#[derive(Debug, Clone, PartialEq)]
pub struct IntCodeMachine {
    pub memory: Memory,
    pub input_pointer: usize,
    pub input_buffer: Vec<Num>,
    pub output_buffer: Vec<Num>,
    pub ip: usize,
    pub blocking: bool,
    pub halt: bool,
    pub iteration: usize,
}

impl IntCodeMachine {
    pub fn new(code: Vec<Num>, input_buffer: Vec<Num>, page_size: usize) -> IntCodeMachine {
        IntCodeMachine {
            ip: 0,
            input_pointer: 0,
            input_buffer,
            output_buffer: vec![],
            memory: Memory::from_vec(page_size, code),
            blocking: false,
            halt: false,
            iteration: 0,
        }
    }
    pub fn execute(&mut self) -> Result<ExecutionStatus, ErrorStatus> {
        // Re-use `modes` rather than create a new one every
        // time in order to avoid memory thrashing.
        let mut modes: Vec<ParamMode> = Vec::with_capacity(5);
        let mut params: Vec<Num> = vec![];

        self.blocking = false;
        loop {
            // There is no 0 opcode, so we can go ahead and
            // report a crash.
            if self.ip >= self.memory.virtual_size() {
                return Err(ErrorStatus::UnterminatedProgram);
            }

            modes.clear(); // important
            let op = deconstruct_opcode(self.memory.read_raw(self.ip)?, &mut modes)?;
            params.clear();
            if op.size() > 1 {
                for i in 0..(op.size() - 1) {
                    params.push(self.memory.read_raw(self.ip + i + 1)?);
                }
            }
            let cons = op.apply(&modes, self)?;
            self.iteration += 1;
            self.ip += cons;
            if self.halt {
                break Ok(ExecutionStatus::Halted);
            }
            if self.blocking {
                break Ok(ExecutionStatus::Blocking);
            }
        }
    }
}
