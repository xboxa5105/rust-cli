use std::process::Command;

pub fn run_command(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_command_on_windows() {
        if cfg!(target_os = "windows") {
            let command = "echo Hello, world!";
            run_command(command);
        }
    }

    #[test]
    fn run_command_on_unix() {
        if !cfg!(target_os = "windows") {
            let command = "echo Hello, world!";
            run_command(command);
        }
    }

    #[test]
    fn run_command_with_error() {
        let command = "invalid_command";
        run_command(command);
    }
}
