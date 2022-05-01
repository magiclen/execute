/*!
# Execute Command Macro

Create `Command` instances using the `command!` macro or the `command_args!` macro.

Also see [`execute`](https://crates.io/crates/execute).

## Examples

```rust
#[macro_use] extern crate execute_command_macro;

let command = command!("program arg1 arg2 'arg 3' -opt1 -opt2");
```

```rust
#[macro_use] extern crate execute_command_macro;

let command = command_args!("program", "arg1", "arg2", "-opt1", "-opt2");
```
*/

/**
Generate the statements at compile time to create a `Command` instance by a command string.

```rust
#[macro_use] extern crate execute_command_macro;

let command = command!("program arg1 arg2 -opt1 -opt2");
```
*/
pub use execute_command_macro_impl::command;

/**
Create a `Command` instance by inputting args separately.

```rust
#[macro_use] extern crate execute_command_macro;

let command = command_args!("program", "arg1", "arg2", "-opt1", "-opt2");
```
*/
#[macro_export]
macro_rules! command_args {
    ($program:expr $(,)*) => {
        ::std::process::Command::new($program)
    };
    ($program:expr, $arg:expr $(, $args:expr)* $(,)*) => {
        {
            let mut command = ::std::process::Command::new($program);

            command.arg(&$arg)$(.arg(&$args))*;

            command
        }
    };
}
