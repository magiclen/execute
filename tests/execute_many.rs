#![cfg(unix)]

extern crate execute;

use std::io::Cursor;
use std::process::{Command, Stdio};

use execute::Execute;

#[test]
fn execute_many() {
    let mut command1 = Command::new("echo");

    command1.arg("abc");

    let mut command2 = Command::new("grep");

    command2.arg("b");

    assert_eq!(Some(0), command1.execute_many(&mut [&mut command2]).unwrap());

    let mut command3 = Command::new("grep");

    command3.arg("d");

    assert_ne!(0, command1.execute_many(&mut [&mut command3]).unwrap().unwrap());
}

#[test]
fn execute_many_output() {
    let mut command1 = Command::new("cat");

    command1.arg("/proc/cpuinfo");

    let mut command2 = Command::new("grep");

    command2.arg("cpu MHz");

    command2.stdout(Stdio::piped());

    let output = command1.execute_many_output(&mut [&mut command2]).unwrap();

    assert!(output.stdout.starts_with(b"cpu MHz"));
}

#[test]
fn execute_many_input() {
    let mut command1 = Command::new("bc");

    let mut command2 = Command::new("grep");

    command2.arg("2");

    assert_eq!(
        Some(0),
        command1.execute_many_input("1 + 1\nquit\n", &mut [&mut command2]).unwrap()
    );
}

#[test]
fn execute_many_input_output() {
    let mut command1 = Command::new("bc");

    let mut command2 = Command::new("grep");

    command2.arg("2");

    command2.stdout(Stdio::piped());

    let output =
        command1.execute_many_input_output("3 + 1\n\n3 - 1\nquit\n", &mut [&mut command2]).unwrap();

    assert_eq!(b"2\n", output.stdout.as_slice());
}

#[test]
fn execute_many_input_reader() {
    let mut command1 = Command::new("bc");

    let mut command2 = Command::new("grep");

    command2.arg("2");

    let mut reader = Cursor::new("1 + 1\nquit\n");

    assert_eq!(
        Some(0),
        command1.execute_many_input_reader(&mut reader, &mut [&mut command2]).unwrap()
    );
}

#[test]
fn execute_many_input_reader_output() {
    let mut command1 = Command::new("bc");

    let mut command2 = Command::new("grep");

    command2.arg("2");

    command2.stdout(Stdio::piped());

    let mut reader = Cursor::new("3 + 1\n\n3 - 1\nquit\n");

    let output =
        command1.execute_many_input_reader_output(&mut reader, &mut [&mut command2]).unwrap();

    assert_eq!(b"2\n", output.stdout.as_slice());
}
