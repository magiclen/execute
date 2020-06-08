/*!
# Execute

This library is for extending `Command` in order to execute programs more easily.

## Usage

```rust
extern crate execute;

use std::process::Command;

use execute::Execute;

// ...
```

### Verify the Program

Since `Command` is used for spawning a process of a command and the executed progrom is external which may not exist or may not be the program that we expected, we usually need to verify the external program at runtime.

The `execute_check_exit_status_code` method can be used to execute a command and check its exit status. For example,

```rust
extern crate execute;

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
extern crate execute;

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
extern crate execute;

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
extern crate execute;

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
extern crate execute;

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
extern crate execute;

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
extern crate execute;

use std::process::{Command, Stdio};
use std::fs::File;

use execute::generic_array::typenum::U4096;
use execute::Execute;

# if cfg!(target_os = "linux") {
let mut cat_command = Command::new("cat");

cat_command.stdout(Stdio::piped());

let mut file = File::open("Cargo.toml").unwrap();

let output = cat_command.execute_input_reader_output2::<U4096>(&mut file).unwrap();

println!("{}", String::from_utf8(output.stdout).unwrap());
# }
```

### Execute Many Commands and Pipe Them Together

```rust
extern crate execute;

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

let output = command1.execute_many_output(&mut [&mut command2, &mut command3]).unwrap();

assert_eq!(b"hello\n", output.stdout.as_slice());
# }
```
*/

pub extern crate generic_array;

use std::io::{self, ErrorKind, Read, Write};
use std::process::{Command, Output, Stdio};

use generic_array::typenum::{IsGreaterOrEqual, True, U1, U256};
use generic_array::{ArrayLength, GenericArray};

pub trait Execute {
    /// Execute this command and get the exit status code. stdin will be set to `Stdio::inherit()`. stdout and stderr will be set to `Stdio::null()`.
    fn execute(&mut self) -> Result<Option<i32>, io::Error>;

    /// Execute this command and get the exit status code. stdin will be set to `Stdio::inherit()`. By default, stdout and stderr are inherited from the parent.
    fn execute_output(&mut self) -> Result<Output, io::Error>;

    /// Execute this command and check the exit status code. stdin will be set to `Stdio::inherit()`. stdout and stderr will be set to `Stdio::null()`. It's usually used for checking whether the program is correct.
    #[inline]
    fn execute_check_exit_status_code(
        &mut self,
        expected_exit_status_code: i32,
    ) -> Result<(), io::Error> {
        match self.execute()? {
            Some(exit_status_code) if exit_status_code == expected_exit_status_code => Ok(()),
            _ => Err(io::Error::new(ErrorKind::Other, "unexpected exit status")),
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
        self.execute_input_reader2::<U256>(reader)
    }

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. stdout and stderr will be set to `Stdio::null()`.
    fn execute_input_reader2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. By default, stdout and stderr are inherited from the parent.
    #[inline]
    fn execute_input_reader_output(&mut self, reader: &mut dyn Read) -> Result<Output, io::Error> {
        self.execute_input_reader_output2::<U256>(reader)
    }

    /// Execute this command and input data from a reader to the process. stdin will be set to `Stdio::piped()`. By default, stdout and stderr are inherited from the parent.
    fn execute_input_reader_output2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Output, io::Error>;

    /// TODO execute_many

    /// Execute this command as well as other commands and pipe their stdin and stdout, and get the exit status code. The stdin of the first process will be set to `Stdio::inherit()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    fn execute_many(&mut self, others: &mut [&mut Command]) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout. The stdin of the first process will be set to `Stdio::inherit()`. By default, the stdout and stderr of the last process are inherited from the parent.
    fn execute_many_output(&mut self, others: &mut [&mut Command]) -> Result<Output, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input in-memory data to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    fn execute_many_input<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input in-memory data to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    fn execute_many_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    #[inline]
    fn execute_many_input_reader(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error> {
        self.execute_many_input_reader2::<U256>(reader, others)
    }

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process, and get the exit status code. The stdin of the first process will be set to `Stdio::piped()`. The stdout and stderr of the last process will be set to `Stdio::null()`.
    fn execute_many_input_reader2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Option<i32>, io::Error>;

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    #[inline]
    fn execute_many_input_reader_output(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error> {
        self.execute_many_input_reader_output2::<U256>(reader, others)
    }

    /// Execute this command as well as other commands and pipe their stdin and stdout, and input data from a reader to the process. The stdin of the first process will be set to `Stdio::piped()`. By default, the stdout and stderr of the last process are inherited from the parent.
    fn execute_many_input_reader_output2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
        others: &mut [&mut Command],
    ) -> Result<Output, io::Error>;
}

impl Execute for Command {
    #[inline]
    fn execute(&mut self) -> Result<Option<i32>, io::Error> {
        self.stdin(Stdio::inherit());
        self.stdout(Stdio::null());
        self.stderr(Stdio::null());

        Ok(self.status()?.code())
    }

    #[inline]
    fn execute_output(&mut self) -> Result<Output, io::Error> {
        self.stdin(Stdio::inherit());

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

        child.stdin.as_mut().unwrap().write_all(data.as_ref())?;

        Ok(child.wait()?.code())
    }

    #[inline]
    fn execute_input_output<D: ?Sized + AsRef<[u8]>>(
        &mut self,
        data: &D,
    ) -> Result<Output, io::Error> {
        self.stdin(Stdio::piped());

        let mut child = self.spawn()?;

        child.stdin.as_mut().unwrap().write_all(data.as_ref())?;

        child.wait_with_output()
    }

    #[inline]
    fn execute_input_reader2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Option<i32>, io::Error> {
        self.stdin(Stdio::piped());
        self.stdout(Stdio::null());
        self.stderr(Stdio::null());

        let mut child = self.spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();

            let mut buffer: GenericArray<u8, N> = GenericArray::default();

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(c) => stdin.write_all(&buffer[0..c])?,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(err),
                }
            }
        }

        Ok(child.wait()?.code())
    }

    #[inline]
    fn execute_input_reader_output2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
        &mut self,
        reader: &mut dyn Read,
    ) -> Result<Output, io::Error> {
        self.stdin(Stdio::piped());

        let mut child = self.spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();

            let mut buffer: GenericArray<u8, N> = GenericArray::default();

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(c) => stdin.write_all(&buffer[0..c])?,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(err),
                }
            }
        }

        child.wait_with_output()
    }

    fn execute_many(&mut self, others: &mut [&mut Command]) -> Result<Option<i32>, io::Error> {
        if others.is_empty() {
            return self.execute();
        }

        self.stdin(Stdio::inherit());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let mut child = self.spawn()?;

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());
        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        Ok(last_other.status()?.code())
    }

    fn execute_many_output(&mut self, others: &mut [&mut Command]) -> Result<Output, io::Error> {
        if others.is_empty() {
            return self.execute_output();
        }

        self.stdin(Stdio::inherit());
        self.stdout(Stdio::piped());
        self.stderr(Stdio::null());

        let mut child = self.spawn()?;

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());

        last_other.spawn()?.wait_with_output()
    }

    fn execute_many_input<D: ?Sized + AsRef<[u8]>>(
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

        let mut child = self.spawn()?;

        child.stdin.as_mut().unwrap().write_all(data.as_ref())?;

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());
        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        Ok(last_other.status()?.code())
    }

    fn execute_many_input_output<D: ?Sized + AsRef<[u8]>>(
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

        let mut child = self.spawn()?;

        child.stdin.as_mut().unwrap().write_all(data.as_ref())?;

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());

        last_other.spawn()?.wait_with_output()
    }

    fn execute_many_input_reader2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
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

        let mut child = self.spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();

            let mut buffer: GenericArray<u8, N> = GenericArray::default();

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(c) => stdin.write_all(&buffer[0..c])?,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(err),
                }
            }
        }

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());
        last_other.stdout(Stdio::null());
        last_other.stderr(Stdio::null());

        Ok(last_other.status()?.code())
    }

    fn execute_many_input_reader_output2<
        N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>,
    >(
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

        let mut child = self.spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();

            let mut buffer: GenericArray<u8, N> = GenericArray::default();

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(c) => stdin.write_all(&buffer[0..c])?,
                    Err(ref err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(err),
                }
            }
        }

        let others_length_dec = others.len() - 1;

        for other in others.iter_mut().take(others_length_dec) {
            other.stdin(child.stdout.unwrap());
            other.stdout(Stdio::piped());
            other.stderr(Stdio::null());

            child = other.spawn()?;
        }

        let last_other = &mut others[others_length_dec];

        last_other.stdin(child.stdout.unwrap());

        last_other.spawn()?.wait_with_output()
    }
}
