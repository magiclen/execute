/*!
# Execute

This library is used for extending `Command` in order to execute programs more easily.

## Usage

```rust
use std::process::Command;

use execute::Execute;

// ...
```

### Verify the Program

Since `Command` is used for spawning a process of a command and the executed progrom is external which may not exist or may not be the program that we expected, we usually need to verify the external program at runtime.

The `execute_check_exit_status_code` method can be used to execute a command and check its exit status. For example,

```rust
use std::process::Command;

use execute::Execute;

const FFMPEG_PATH: &str = "/path/to/ffmpeg";

let mut first_command = Command::new(FFMPEG_PATH);

first_command.arg("-version");

if first_command.execute_check_exit_status_code(0).is_err() {
    eprintln!("The path `{}` is not a correct FFmpeg executable binary file.", FFMPEG_PATH);
}
```

### Execute and Get the Exit Status

```rust,ignore
use std::process::Command;

use execute::Execute;

const FFMPEG_PATH: &str = "/path/to/ffmpeg";

let mut command = Command::new(FFMPEG_PATH);

command.arg("-i");
command.arg("/path/to/media-file");
command.arg("/path/to/output-file");

if let Some(exit_code) = command.execute().unwrap() {
    if exit_code == 0 {
        println!("Ok.");
    } else {
        eprintln!("Failed.");
    }
} else {
    eprintln!("Interrupted!");
}
```

### Execute and Get the Output

#### Output to the Screen

```rust,ignore
use std::process::Command;

use execute::Execute;

const FFMPEG_PATH: &str = "/path/to/ffmpeg";

let mut command = Command::new(FFMPEG_PATH);

command.arg("-i");
command.arg("/path/to/media-file");
command.arg("/path/to/output-file");

let output = command.execute_output().unwrap();

if let Some(exit_code) = output.status.code() {
    if exit_code == 0 {
        println!("Ok.");
    } else {
        eprintln!("Failed.");
    }
} else {
    eprintln!("Interrupted!");
}
```

#### Output to Memory (Captured)

```rust,ignore
use std::process::{Command, Stdio};

use execute::Execute;

const FFMPEG_PATH: &str = "/path/to/ffmpeg";

let mut command = Command::new(FFMPEG_PATH);

command.arg("-i");
command.arg("/path/to/media-file");
command.arg("/path/to/output-file");

command.stdout(Stdio::piped());
command.stderr(Stdio::piped());

let output = command.execute_output().unwrap();

if let Some(exit_code) = output.status.code() {
    if exit_code == 0 {
        println!("Ok.");
    } else {
        eprintln!("Failed.");
    }
} else {
    eprintln!("Interrupted!");
}

println!("{}", String::from_utf8(output.stdout).unwrap());
println!("{}", String::from_utf8(output.stderr).unwrap());
```

### Execute and Input Data

#### Input In-memory Data

```rust
use std::process::{Command, Stdio};

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut bc_command = Command::new("bc");

bc_command.stdout(Stdio::piped());

let output = bc_command.execute_input_output("2^99\n").unwrap();

println!("Answer: {}", String::from_utf8(output.stdout).unwrap().trim_end());
# }
```

#### Input from a Reader

```rust
use std::process::{Command, Stdio};
use std::fs::File;

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut cat_command = Command::new("cat");

cat_command.stdout(Stdio::piped());

let mut file = File::open("Cargo.toml").unwrap();

let output = cat_command.execute_input_reader_output(&mut file).unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

By default, the buffer size is 256 bytes. If you want to change that, you can use the `_reader_output2` or `_reader2` methods and define a length explicitly.

For example, to change the buffer size to 4096 bytes,

```rust
use std::process::{Command, Stdio};
use std::fs::File;

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut cat_command = Command::new("cat");

cat_command.stdout(Stdio::piped());

let mut file = File::open("Cargo.toml").unwrap();

let output = cat_command.execute_input_reader_output2::<4096>(&mut file).unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

### Execute Multiple Commands and Pipe Them Together

```rust
use std::process::{Command, Stdio};

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut command1 = Command::new("echo");
command1.arg("HELLO WORLD");

let mut command2 = Command::new("cut");
command2.arg("-d").arg(" ").arg("-f").arg("1");

let mut command3 = Command::new("tr");
command3.arg("A-Z").arg("a-z");

command3.stdout(Stdio::piped());

let output = command1.execute_multiple_output(&mut [&mut command2, &mut command3]).unwrap();

assert_eq!(b"hello\n", output.stdout.as_slice());
# }
```

### Run a Command String in the Current Shell

The `shell` function can be used to create a `Command` instance with a single command string instead of a program name and scattered arguments.

```rust
use std::process::{Command, Stdio};

use execute::{Execute, shell};

# if cfg!(target_os = "linux") {
let mut command = shell("cat /proc/meminfo");

command.stdout(Stdio::piped());

let output = command.execute_output().unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

### Parse a Command String at Runtime

The `command` function can be used to create a `Command` instance with a single command string instead of a program name and scattered arguments. The difference between the `shell` function and the `command` function is that the former is interpreted by the current shell while the latter is parsed by this crate.

```rust
use std::process::{Command, Stdio};

use execute::{Execute, command};

# if cfg!(target_os = "linux") {
let mut command = command("cat '/proc/meminfo'");

command.stdout(Stdio::piped());

let output = command.execute_output().unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

### Parse a Command String at Compile Time

The `command!` macro can be used to create a `Command` instance with a single command string literal instead of a program name and scattered arguments.

```rust
use std::process::{Command, Stdio};

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut command = execute::command!("cat '/proc/meminfo'");

command.stdout(Stdio::piped());

let output = command.execute_output().unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

### Create a `Command` Instance by Providing Arguments Separately

The `command_args!` macro can be used to create a `Command` instance with a program name and arguments separately. The program name and arguments can be non-literal.

```rust
use std::process::{Command, Stdio};

use execute::Execute;

# if cfg!(target_os = "linux") {
let mut command = execute::command_args!("cat", "/proc/meminfo");

command.stdout(Stdio::piped());

let output = command.execute_output().unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```
*/

#[cfg(unix)]
use std::{env, ffi::OsString};
use std::{
    ffi::OsStr,
    io::{self, ErrorKind, Read, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Output, Stdio},
    thread,
};

pub use execute_command_macro::{command, command_args};
use execute_command_tokens::command_tokens;

const DEFAULT_READER_BUFFER_SIZE: usize = 256;

#[inline]
fn take_child_stdin(child: &mut Child) -> Result<ChildStdin, io::Error> {
    child.stdin.take().ok_or_else(|| io::Error::other("child stdin was not piped"))
}

#[inline]
fn take_child_stdout(child: &mut Child) -> Result<ChildStdout, io::Error> {
    child.stdout.take().ok_or_else(|| io::Error::other("child stdout was not piped"))
}

#[inline]
fn write_stdin<D: ?Sized + AsRef<[u8]>>(mut stdin: ChildStdin, data: &D) -> Result<(), io::Error> {
    stdin.write_all(data.as_ref())
}

fn copy_reader_to_stdin<const N: usize>(
    mut stdin: ChildStdin,
    reader: &mut dyn Read,
) -> Result<(), io::Error> {
    const { assert!(N > 0, "reader buffer size must be greater than zero") };

    let mut buffer = [0u8; N];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(c) => stdin.write_all(&buffer[0..c])?,
            Err(ref err) if err.kind() == ErrorKind::Interrupted => (),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

fn wait_with_stdin_writer<T, W, F>(wait: W, write_stdin: F) -> Result<T, io::Error>
where
    T: Send,
    W: FnOnce() -> Result<T, io::Error> + Send,
    F: FnOnce() -> Result<(), io::Error>, {
    thread::scope(|scope| {
        // Waiting in a scoped thread lets the child drain its stdout and stderr while this thread is still writing stdin.
        let wait_handle = scope.spawn(wait);
        let write_result = write_stdin();
        let wait_result = match wait_handle.join() {
            Ok(result) => result,
            Err(_) => Err(io::Error::other("child wait thread panicked")),
        };

        match write_result {
            Ok(()) => wait_result,
            Err(err) => Err(err),
        }
    })
}

fn kill_and_wait_children(mut children: Vec<Child>) {
    // If pipeline setup fails, terminate every child we already spawned and reap them before returning the setup error.
    for child in &mut children {
        let _ = child.kill();
    }

    for mut child in children {
        let _ = child.wait();
    }
}

fn wait_upstream_children(children: Vec<Child>) -> Result<(), io::Error> {
    let mut first_error = None;

    for mut child in children {
        if let Err(err) = child.wait() {
            if first_error.is_none() {
                first_error = Some(err);
            }
        }
    }

    match first_error {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn finish_pipeline_result<T>(
    result: Result<T, io::Error>,
    upstream_children: Vec<Child>,
) -> Result<T, io::Error> {
    // The public result follows normal shell pipeline behavior and comes from the last command, but upstream children still must be reaped.
    let upstream_result = wait_upstream_children(upstream_children);

    match result {
        Ok(value) => {
            upstream_result?;

            Ok(value)
        },
        Err(err) => Err(err),
    }
}

fn spawn_pipeline(
    first: &mut Command,
    others: &mut [&mut Command],
) -> Result<(Vec<Child>, Child), io::Error> {
    let mut upstream_children = Vec::with_capacity(others.len());
    let mut previous_child = first.spawn()?;
    let last_index = others.len() - 1;

    for other in others.iter_mut().take(last_index) {
        // Each child becomes upstream as soon as its stdout is moved into the next command's stdin.
        let stdout = match take_child_stdout(&mut previous_child) {
            Ok(stdout) => stdout,
            Err(err) => {
                upstream_children.push(previous_child);
                kill_and_wait_children(upstream_children);

                return Err(err);
            },
        };

        other.stdin(stdout);
        other.stdout(Stdio::piped());
        other.stderr(Stdio::null());

        upstream_children.push(previous_child);

        previous_child = match other.spawn() {
            Ok(child) => child,
            Err(err) => {
                kill_and_wait_children(upstream_children);

                return Err(err);
            },
        };
    }

    let stdout = match take_child_stdout(&mut previous_child) {
        Ok(stdout) => stdout,
        Err(err) => {
            upstream_children.push(previous_child);
            kill_and_wait_children(upstream_children);

            return Err(err);
        },
    };

    let last_other = &mut others[last_index];

    last_other.stdin(stdout);
    upstream_children.push(previous_child);

    // Keep the last child separate because callers need its status or captured output as the method result.
    match last_other.spawn() {
        Ok(last_child) => Ok((upstream_children, last_child)),
        Err(err) => {
            kill_and_wait_children(upstream_children);

            Err(err)
        },
    }
}

pub trait Execute {
    /// Execute this command and get the exit status code. stdout and stderr will be set to `Stdio::null()`. By default, stdin is inherited from the parent.
    fn execute(&mut self) -> Result<Option<i32>, io::Error>;

    /// Execute this command and get the exit status code. By default, stdin, stdout and stderr are inherited from the parent.
    fn execute_output(&mut self) -> Result<Output, io::Error>;

    /// Execute this command and check the exit status code. stdout and stderr will be set to `Stdio::null()`. By default, stdin is inherited from the parent. It's usually used for checking whether the program is correct.
    #[inline]
    fn execute_check_exit_status_code(
        &mut self,
        expected_exit_status_code: i32,
    ) -> Result<(), io::Error> {
        match self.execute()? {
            Some(exit_status_code) if exit_status_code == expected_exit_status_code => Ok(()),
            _ => Err(io::Error::other("unexpected exit status")),
        }
    }

    /// Execute this command and input in-memory data to the process. stdin will be set to `Stdio::piped()`. stdout and stderr will be set to `Stdio::null()`.
    fn execute_input<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command and input in-memory data to the process. stdin will be set to `Stdio::piped()`. By default, stdout and stderr are inherited from the parent.
    fn execute_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
    ) -> Result<Output, io::Error>;

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. stdout and stderr will be set to `Stdio::null()`.
    #[inline]
    fn execute_input_reader(&mut self, reader: &mut dyn Read) -> Result<Option<i32>, io::Error> {
        self.execute_input_reader2::<DEFAULT_READER_BUFFER_SIZE>(reader)
    }

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. stdout and stderr will be set to `Stdio::null()`.
    fn execute_input_reader2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. By default, stdout and stderr are inherited from the parent.
    #[inline]
    fn execute_input_reader_output(&mut self, reader: &mut dyn Read) -> Result<Output, io::Error> {
        self.execute_input_reader_output2::<DEFAULT_READER_BUFFER_SIZE>(reader)
    }

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. By default, stdout and stderr are inherited from the parent.
    fn execute_input_reader_output2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Output, io::Error>;

    // TODO execute_multiple

    /// Execute this command as well as other commands and pipe their stdin and stdout, and get the exit status code. The stdout and stderr of the last process will be set to `Stdio::null()`. By default, the stdin of the first process is inherited from the parent.
    fn execute_multiple(&mut self, others: &mut [&mut Command]) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout. By default, the stdin of the first process, the stdout and stderr of the last process are inherited from the parent.
    fn execute_multiple_output(&mut self, others: &mut [&mut Command])
    -> Result<Output, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input in-memory data to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    fn execute_multiple_input<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input in-memory data to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    fn execute_multiple_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    #[inline]
    fn execute_multiple_input_reader(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error> {
        self.execute_multiple_input_reader2::<DEFAULT_READER_BUFFER_SIZE>(reader, others)
    }

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    fn execute_multiple_input_reader2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    #[inline]
    fn execute_multiple_input_reader_output(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error> {
        self.execute_multiple_input_reader_output2::<DEFAULT_READER_BUFFER_SIZE>(reader, others)
    }

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    fn execute_multiple_input_reader_output2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error>;
}

impl Execute for Command {
    #[inline]
    fn execute(&mut self) -> Result<Option<i32>, io::Error> {
        self.stdout(Stdio::null());
        self.stderr(Stdio::null());

        Ok(self.status()?.code())
    }

    #[inline]
    fn execute_output(&mut self) -> Result<Output, io::Error> {
        self.spawn()?.wait_with_output()
    }

    #[inline]
    fn execute_input<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
    ) -> Result<Option<i32>, io::Error> {
        self.stdin(Stdio::piped());
        self.stdout(Stdio::null());
        self.stderr(Stdio::null());

        let mut child = self.spawn()?;
        let stdin = take_child_stdin(&mut child)?;

        wait_with_stdin_writer(
            move || child.wait().map(|status| status.code()),
            || write_stdin(stdin, data),
        )
    }

    #[inline]
    fn execute_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
    ) -> Result<Output, io::Error> {
        self.stdin(Stdio::piped());

        let mut child = self.spawn()?;
        let stdin = take_child_stdin(&mut child)?;

        wait_with_stdin_writer(move || child.wait_with_output(), || write_stdin(stdin, data))
    }

    #[inline]
    fn execute_input_reader2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Option<i32>, io::Error> {
        self.stdin(Stdio::piped());
        self.stdout(Stdio::null());
        self.stderr(Stdio::null());

        let mut child = self.spawn()?;
        let stdin = take_child_stdin(&mut child)?;

        wait_with_stdin_writer(
            move || child.wait().map(|status| status.code()),
            || copy_reader_to_stdin::<N>(stdin, reader),
        )
    }

    #[inline]
    fn execute_input_reader_output2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Output, io::Error> {
        self.stdin(Stdio::piped());

        let mut child = self.spawn()?;
        let stdin = take_child_stdin(&mut child)?;

        wait_with_stdin_writer(
            move || child.wait_with_output(),
            || copy_reader_to_stdin::<N>(stdin, reader),
        )
    }

    fn execute_multiple(&mut self, others: &mut [&mut Command]) -> Result<Option<i32>, io::Error> {
        if others.is_empty() {
            return self.execute();
        }

        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let others_length_dec = others.len() - 1;
        let last_other = &mut others[others_length_dec];

        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        let (upstream_children, mut last_child) = spawn_pipeline(self, others)?;
        let status_result = last_child.wait().map(|status| status.code());

        finish_pipeline_result(status_result, upstream_children)
    }

    fn execute_multiple_output(
        &mut self,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error> {
        if others.is_empty() {
            return self.execute_output();
        }

        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let (upstream_children, last_child) = spawn_pipeline(self, others)?;
        let output_result = last_child.wait_with_output();

        finish_pipeline_result(output_result, upstream_children)
    }

    fn execute_multiple_input<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error> {
        if others.is_empty() {
            return self.execute_input(data);
        }

        self.stdin(Stdio::piped());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let others_length_dec = others.len() - 1;
        let last_other = &mut others[others_length_dec];

        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        let (mut upstream_children, mut last_child) = spawn_pipeline(self, others)?;
        let stdin = take_child_stdin(&mut upstream_children[0])?;
        let status_result = wait_with_stdin_writer(
            move || last_child.wait().map(|status| status.code()),
            || write_stdin(stdin, data),
        );

        finish_pipeline_result(status_result, upstream_children)
    }

    fn execute_multiple_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error> {
        if others.is_empty() {
            return self.execute_input_output(data);
        }

        self.stdin(Stdio::piped());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let (mut upstream_children, last_child) = spawn_pipeline(self, others)?;
        let stdin = take_child_stdin(&mut upstream_children[0])?;
        let output_result = wait_with_stdin_writer(
            move || last_child.wait_with_output(),
            || write_stdin(stdin, data),
        );

        finish_pipeline_result(output_result, upstream_children)
    }

    fn execute_multiple_input_reader2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error> {
        if others.is_empty() {
            return self.execute_input_reader2::<N>(reader);
        }

        self.stdin(Stdio::piped());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let others_length_dec = others.len() - 1;
        let last_other = &mut others[others_length_dec];

        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        let (mut upstream_children, mut last_child) = spawn_pipeline(self, others)?;
        let stdin = take_child_stdin(&mut upstream_children[0])?;
        let status_result = wait_with_stdin_writer(
            move || last_child.wait().map(|status| status.code()),
            || copy_reader_to_stdin::<N>(stdin, reader),
        );

        finish_pipeline_result(status_result, upstream_children)
    }

    fn execute_multiple_input_reader_output2<const N: usize>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error> {
        if others.is_empty() {
            return self.execute_input_reader_output2::<N>(reader);
        }

        self.stdin(Stdio::piped());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let (mut upstream_children, last_child) = spawn_pipeline(self, others)?;
        let stdin = take_child_stdin(&mut upstream_children[0])?;
        let output_result = wait_with_stdin_writer(
            move || last_child.wait_with_output(),
            || copy_reader_to_stdin::<N>(stdin, reader),
        );

        finish_pipeline_result(output_result, upstream_children)
    }
}

/// Create a `Command` instance which can be executed by the current command language interpreter (shell).
#[cfg(unix)]
#[inline]
pub fn shell<S: AsRef<OsStr>>(cmd: S) -> Command {
    use std::sync::LazyLock;

    static SHELL: LazyLock<OsString> = LazyLock::new(|| {
        env::var_os("SHELL").unwrap_or_else(|| OsString::from(String::from("sh")))
    });

    let mut command = Command::new(&*SHELL);

    command.arg("-c");
    command.arg(cmd);

    command
}

/// Create a `Command` instance which can be executed by the current command language interpreter (shell).
#[cfg(windows)]
#[inline]
pub fn shell<S: AsRef<OsStr>>(cmd: S) -> Command {
    let mut command = Command::new("cmd.exe");

    command.arg("/c");
    command.arg(cmd);

    command
}

/// Create a `Command` instance by parsing a command string.
#[inline]
pub fn command<S: AsRef<str>>(cmd: S) -> Command {
    let tokens = command_tokens(cmd);

    if tokens.is_empty() {
        Command::new("")
    } else {
        let mut command = Command::new(&tokens[0]);

        command.args(&tokens[1..]);

        command
    }
}
