use std::{collections::HashMap, process::Command};

pub fn is_wsl() -> bool {
    let output = Command::new("bash")
        .arg("-c")
        .arg("grep -qi microsoft /proc/version")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

pub fn split_args(command: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut parts = command.split_whitespace();
    while let Some(part) = parts.next() {
        args.push(part.to_string());
    }
    args
}

pub fn template_replace(template: &str, params: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in params {
        result = result.replace(key, value);
    }
    result
}
pub fn template_replace_single(template: &str, key: &str, value: &str) -> String {
    let mut result = template.to_string();
    let key = format!("${{{}}}", key); // key=output时，结果为：${output}
    result = result.replace(&key, value);
    result
}
