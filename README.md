## teepee <img src="assets/teepee.png" width="150" height="150" align="right" />

[![Crates.io](https://img.shields.io/crates/v/teepee)](https://crates.io/crates/teepee)
[![Docs.rs](https://docs.rs/teepee/badge.svg)](https://docs.rs/teepee)

Wrap your command in a teepee and capture its output while also piping it to the parent process
`stdout` and `stderr`.

```rs
use std::process::Command;
use teepee::Teepee;

fn main() {
    let mut command = Command::new("ls");
    command.arg("-l").arg("-a");

    let mut teepee_cmd = Teepee::new(command);
    eprintln!("\nCommand running ...\n");
    match teepee_cmd.output() {
        Ok(output) => {
            eprintln!("\nCommand executed successfully.\n");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "Captured stdout (lines: {}):\n> {}",
                stdout.lines().count(),
                stdout.lines().collect::<Vec<&str>>().join("\n> ")
            );
            eprintln!(
                "Captured stderr (lines: {}):\n{}",
                stderr.lines().count(),
                stderr
            );
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
        }
    }
}
```

This example prints the output of the `ls -l -a` command while it is running and then again the
captured output prefixed by `> `.

<details>
<summary>Output</summary>

```
❯ cargo run --example ls
   Compiling teepee v0.0.1 (/Volumes/d/dev/rust/teepee)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
     Running `target/debug/examples/ls`

Command running ...

total 40
drwxr-xr-x  12 thlorenz  admin   384 Nov 20 14:35 .
drwxr-xr-x   5 thlorenz  admin   160 Nov 20 14:17 ..
drwxr-xr-x   9 thlorenz  admin   288 Nov 20 14:35 .git
-rw-r--r--   1 thlorenz  admin     8 Nov 20 14:17 .gitignore
-rw-r--r--   1 thlorenz  admin   150 Nov 20 14:35 Cargo.lock
-rw-r--r--   1 thlorenz  admin   110 Nov 20 14:35 Cargo.toml
-rw-r--r--   1 thlorenz  admin  1456 Nov 20 14:35 LICENSE
-rw-r--r--   1 thlorenz  admin  1457 Nov 20 14:38 README.md
drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:26 assets
drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:23 examples
drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:17 src
drwxr-xr-x@  5 thlorenz  admin   160 Nov 20 14:18 target

Command executed successfully.

Captured stdout (lines: 13):
> total 40
> drwxr-xr-x  12 thlorenz  admin   384 Nov 20 14:35 .
> drwxr-xr-x   5 thlorenz  admin   160 Nov 20 14:17 ..
> drwxr-xr-x   9 thlorenz  admin   288 Nov 20 14:35 .git
> -rw-r--r--   1 thlorenz  admin     8 Nov 20 14:17 .gitignore
> -rw-r--r--   1 thlorenz  admin   150 Nov 20 14:35 Cargo.lock
> -rw-r--r--   1 thlorenz  admin   110 Nov 20 14:35 Cargo.toml
> -rw-r--r--   1 thlorenz  admin  1456 Nov 20 14:35 LICENSE
> -rw-r--r--   1 thlorenz  admin  1457 Nov 20 14:38 README.md
> drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:26 assets
> drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:23 examples
> drwxr-xr-x   3 thlorenz  admin    96 Nov 20 14:17 src
> drwxr-xr-x@  5 thlorenz  admin   160 Nov 20 14:18 target
Captured stderr (lines: 0):
```
</details>

## LICENSE

MIT
