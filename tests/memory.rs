use intcode::{Memory, ParamMode};
#[test]
fn from_vec() {
    let mem = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut mem_out = mem.clone();
    mem_out.resize_with(12, || 0);
    assert_eq!(
        Memory::from_vec(2, mem.clone()).flatten(),
        mem_out,
        "Doesn't map a vector to pages correctly"
    );
}
#[test]
fn read_raw() {
    let mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(mem.read_raw(0), Ok(1), "Fails to read from memory");
}
#[test]
fn write_raw() {
    let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.write_raw(0, 5).unwrap();
    assert_eq!(mem.flatten()[0], 5, "Fails to write to memory");
}
#[test]
fn read_position() {
    let mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        mem.read(0, ParamMode::Position),
        Ok(2),
        "Fails to write to memory"
    );
}
#[test]
fn read_relative() {
    let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.relative_base = 1;
    assert_eq!(
        mem.read(1, ParamMode::Relative),
        Ok(4),
        "Fails to write to memory"
    );
}
#[test]
fn write_position() {
    let mut mem = Memory::from_vec(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.write(0, 5, ParamMode::Position).unwrap();
    assert_eq!(mem.read_raw(1), Ok(5), "Fails to write in positional mode");
}
