use std::io::{self, BufRead, Write};
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread;

/// Wrapper around `std::process::Command` that captures both stdout and stderr while
/// piping them to the parent process in real-time as well.
pub struct Teepee {
    command: Command,
}

impl Teepee {
    /// Create a new `Teepee` instance wrapping the given `Command`.
    pub fn new(command: Command) -> Self {
        Self { command }
    }

    /// Spawns the wrapped command and capture both stdout and stderr while running
    /// it to completion. Returns an `Output` struct containing the captured data.
    /// Durint the command's execution all stderr and stdout output is piped to the
    /// parent process as well.
    pub fn output(&mut self) -> io::Result<Output> {
        let mut child = self
            .command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let (stdout_sender, stdout_receiver) = channel();
        let (stderr_sender, stderr_receiver) = channel();

        thread::spawn(move || {
            tee_reader(stdout, stdout_sender);
        });

        thread::spawn(move || {
            tee_reader(stderr, stderr_sender);
        });

        let status = child.wait()?;

        let stdout = stdout_receiver.recv().unwrap();
        let stderr = stderr_receiver.recv().unwrap();

        Ok(Output {
            status,
            stdout,
            stderr,
        })
    }
}

fn tee_reader<R>(reader: R, sender: Sender<Vec<u8>>)
where
    R: io::Read,
{
    let mut buffer = Vec::new();
    let mut reader = io::BufReader::new(reader);

    loop {
        let mut chunk = Vec::new();
        match reader.read_until(b'\n', &mut chunk) {
            Ok(0) => break,
            Ok(_) => {
                io::stdout().write_all(&chunk).unwrap();
                buffer.extend_from_slice(&chunk);
            }
            Err(_) => break,
        }
    }

    sender.send(buffer).unwrap();
}
