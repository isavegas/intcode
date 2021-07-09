use crate::to_num;
use crate::ErrorStatus;
use crate::IntCodeMachine;
use crate::Num;

// opcode is in format EDCBA where BA is 2 digit opcode, C is first
// parameter mode, D is second, E is third, etc (flipped from AoC).
// Note that currently we support 0 for relative, 1 for immediate,
// and 2 for relative mode.
// We use bit masks currently to detect whether immediate mode is
// active for a given parameter.
// DOES NOT CLEAR MODES.
pub fn deconstruct_opcode(opcode: Num, modes: &mut Vec<ParamMode>) -> Result<Op, ErrorStatus> {
    let op = opcode - ((opcode / 100) * 100);

    let mut o = opcode / 100;
    while o > 0 {
        if o % 10 == 1 {
            modes.push(ParamMode::Immediate);
        } else if o % 10 == 2 {
            modes.push(ParamMode::Relative);
        } else {
            modes.push(ParamMode::Position);
        }
        o /= 10;
    }

    // ^ won't detect implicit leading zeros, so we push until we have minimum parameters.
    // alternatively, I could refactor my apply() logic to handle missing parameter options.
    // Definitely a TODO, especially with zero cost abstractions.
    while modes.len() < 3 {
        modes.push(ParamMode::Position);
    }

    Op::new(op)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamMode {
    Immediate,
    Relative,
    Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    val: Num,
    mode: ParamMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Input,
    Output,
    JNZ,
    JZ,
    LT,
    EQ,
    ARB,
    Halt,
}

impl Op {
    pub fn new(n: Num) -> Result<Op, ErrorStatus> {
        match n {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
            3 => Ok(Op::Input),
            4 => Ok(Op::Output),
            /* These might change opcodes. */
            5 => Ok(Op::JNZ),
            6 => Ok(Op::JZ),
            7 => Ok(Op::LT),
            8 => Ok(Op::EQ),
            9 => Ok(Op::ARB),
            99 => Ok(Op::Halt),
            _ => Err(ErrorStatus::UnrecognizedOp(n)),
        }
    }

    // returns the amount of opcodes consumed. Should always be at least 1
    pub fn apply(
        &self,
        modes: &[ParamMode],
        machine: &mut IntCodeMachine,
    ) -> Result<usize, ErrorStatus> {
        macro_rules! mem_write {
            ($offset:expr, $value:expr) => {{
                let o = $offset;
                machine.memory.write(machine.ip + o, $value, modes[o - 1])
            }};
        }
        macro_rules! mem_read {
            ($offset:expr) => {{
                let o = $offset;
                machine.memory.read(machine.ip + o, modes[o - 1])
            }};
        }
        let mut params_f = vec![];
        for n in 1..self.size() {
            params_f.push(machine.memory.read_raw(machine.ip + n)?);
        }

        //println!("{} :: {:?} :: {:?} :: {:?}", machine.ip, self, params_f, modes);
        match self {
            Op::Add => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, p1 + p2)?;
            }
            Op::Mul => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, p1 * p2)?;
            }
            Op::Input => {
                if let Some(n) = machine.input_buffer.get(machine.input_pointer).cloned() {
                    mem_write!(1, n)?;
                    machine.input_pointer += 1;
                } else {
                    machine.blocking = true;
                    // Tell the VM not to increment IP
                    return Ok(0);
                }
            }
            Op::Output => {
                machine.output_buffer.push(mem_read!(1)?);
            }
            Op::JNZ => {
                let p1 = mem_read!(1)?;
                let target = mem_read!(2)?;
                match p1 != 0 {
                    true => {
                        machine.ip = target as usize;
                        return Ok(0);
                    }
                    false => {
                        return Ok(self.size());
                    }
                }
            }
            Op::JZ => {
                let p1 = mem_read!(1)?;
                let target = mem_read!(2)?;
                match p1 == 0 {
                    true => {
                        machine.ip = target as usize;
                        return Ok(0);
                    }
                    false => {
                        return Ok(self.size());
                    }
                }
            }
            Op::LT => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, to_num(p1 < p2))?;
            }
            Op::EQ => {
                let p1 = mem_read!(1)?;
                let p2 = mem_read!(2)?;
                mem_write!(3, to_num(p1 == p2))?;
            }
            Op::ARB => {
                let p1 = mem_read!(1)?;
                machine.memory.adjust_relative_base(p1)?;
            }
            Op::Halt => machine.halt = true,
        }

        Ok(self.size())
    }

    pub fn size(&self) -> usize {
        match self {
            Op::Add => 4,
            Op::Mul => 4,
            Op::Input => 2,
            Op::Output => 2,
            Op::JNZ => 3,
            Op::JZ => 3,
            Op::LT => 4,
            Op::EQ => 4,
            Op::ARB => 2,
            Op::Halt => 1,
        }
    }
}
