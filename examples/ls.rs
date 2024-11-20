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
