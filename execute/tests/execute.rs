#![cfg(target_os = "linux")]

use std::{
    io::Cursor,
    process::{Command, Stdio},
};

use execute::Execute;

#[test]
fn execute() {
    let mut command = Command::new("true");

    assert_eq!(Some(0), command.execute().unwrap());

    let mut command = Command::new("false");

    assert_ne!(0, command.execute().unwrap().unwrap());
}

#[test]
fn execute_output() {
    let mut command = Command::new("cat");

    command.arg("/proc/cpuinfo");

    command.stdout(Stdio::piped());

    let output = command.execute_output().unwrap();

    assert!(output.stdout.starts_with(b"processor"));
}

#[test]
fn execute_check_exit_status_code() {
    let mut command = Command::new("true");

    assert!(command.execute_check_exit_status_code(0).is_ok());

    let mut command = Command::new("false");

    assert!(command.execute_check_exit_status_code(0).is_err());
}

#[test]
fn execute_input() {
    let mut command = Command::new("bc");

    assert_eq!(Some(0), command.execute_input("1 + 1\n").unwrap());
}

#[test]
fn execute_input_output() {
    let mut command = Command::new("bc");

    command.stdout(Stdio::piped());

    let output = command.execute_input_output("1 + 1\n").unwrap();

    assert_eq!(b"2\n", output.stdout.as_slice());
}

#[test]
fn execute_input_reader() {
    let mut command = Command::new("bc");

    let mut reader = Cursor::new("1 + 1\n");

    assert_eq!(Some(0), command.execute_input_reader(&mut reader).unwrap());
}

#[test]
fn execute_input_reader_output() {
    let mut command = Command::new("bc");

    command.stdout(Stdio::piped());

    let mut reader = Cursor::new("1 + 1\n");

    let output = command.execute_input_reader_output(&mut reader).unwrap();

    assert_eq!(b"2\n", output.stdout.as_slice());
}
