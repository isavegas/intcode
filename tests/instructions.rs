use intcode::{parse_intcode, ErrorStatus, ExecutionStatus, IntCodeMachine, Num};

fn execute(intcode: &str) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
    run_code(parse_intcode(intcode).unwrap(), vec![])
}

fn run_code(
    intcode: Vec<Num>,
    input: Vec<Num>,
) -> (IntCodeMachine, Result<ExecutionStatus, ErrorStatus>) {
    let mut machine = IntCodeMachine::new(intcode, input, 100);
    let r = machine.execute();
    (machine, r)
}

#[test]
fn add_position() {
    let (machine, result) = execute("1,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        6,
        "Fails to add positional parameters"
    )
}

#[test]
fn add_immediate() {
    let (machine, result) = execute("1101,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        10,
        "Fails to add immediate parameters"
    )
}

#[test]
fn add_mixed() {
    let (machine, result) = execute("1001,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        8,
        "Fails to add mixed mode parameters"
    )
}

#[test]
fn add_immediate_output_param() {
    let (machine, result) = execute("10001,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        6,
        "Fails to ignore mode for add output parameter"
    )
}

#[test]
fn mul_position() {
    let (machine, result) = execute("2,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        9,
        "Fails to multiply positional parameters"
    )
}

#[test]
fn mul_immediate() {
    let (machine, result) = execute("1102,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        25,
        "Fails to multiply immediate parameters"
    )
}

#[test]
fn mul_mixed() {
    let (machine, result) = execute("1002,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        15,
        "Fails to multiply mixed mode parameters"
    )
}

#[test]
fn mul_immediate_output_param() {
    let (machine, result) = execute("10002,5,5,0,99,3");
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        9,
        "Fails to ignore mode for mul output parameter"
    )
}

#[test]
fn input() {
    let (machine, result) = run_code(parse_intcode("3,0,99").unwrap(), vec![4]);
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.memory.read_raw(0).unwrap(),
        4,
        "Fails to get existing input data without blocking"
    )
}

#[test]
fn output() {
    let (machine, result) = run_code(parse_intcode("4,0,99").unwrap(), vec![]);
    assert!(result.is_ok(), "Crashed! {:?}", result.unwrap_err(),);
    assert_eq!(
        machine.output_buffer.get(0),
        Some(&4_isize),
        "Fails to get existing input data without blocking"
    );
}
