pub type Num = isize;

mod memory;
pub use memory::Memory;

mod machine;
pub use machine::IntCodeMachine;

mod op;
pub use op::{deconstruct_opcode, Op, Operand, ParamMode};

mod status;
pub use status::{ErrorStatus, ExecutionStatus};

pub fn parse_intcode(s: &str) -> Result<Vec<Num>, std::string::ParseError> {
    let mut intcode = vec![];
    for v in s.trim().split(',') {
        intcode.push(v.parse::<Num>().unwrap());
    }
    Ok(intcode)
}

#[inline(always)]
fn to_num(b: bool) -> Num {
    match b {
        true => 1,
        false => 0,
    }
}
