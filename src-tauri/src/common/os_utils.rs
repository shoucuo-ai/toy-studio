use std::process::Command;

pub fn is_wsl() -> bool {
    let output = Command::new("bash")
        .arg("-c")
        .arg("grep -qi microsoft /proc/version")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

