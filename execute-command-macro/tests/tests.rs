#![cfg(target_os = "linux")]

#[macro_use]
extern crate execute_command_macro;

#[test]
fn command(){
    let mut command = command!("sh -c \"echo '123 456' | cut -d ' ' -f 1\"");

    let output = command.output().unwrap();

    assert_eq!(b"123\n", output.stdout.as_slice());
}