use lazy_static::lazy_static;
use intcode::{parse_intcode, IntCodeMachine, Num};

// We don't want to include time to parse intcode in the benchmarks.
lazy_static! {
    static ref INTCODE: Vec<Num> =
        parse_intcode(include_str!("./code/day_09.txt")).expect("Invalid intcode");
}

#[inline(never)]
pub fn part_1(f: usize) {
    IntCodeMachine::new(INTCODE.clone(), vec![1], f).execute().expect("Error running machine!");
}

#[inline(never)]
pub fn part_2(f: usize) {
    IntCodeMachine::new(INTCODE.clone(), vec![2], f).execute().expect("Error running machine!");
}
