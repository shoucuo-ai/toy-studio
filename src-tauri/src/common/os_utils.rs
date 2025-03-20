use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use crate::APP_INSTALLED;

pub fn is_wsl() -> bool {
    match std::env::consts::OS {
        "linux" => {
            let output = Command::new("bash")
                .arg("-c")
                .arg("grep -qi microsoft /proc/version")
                .output()
                .expect("Failed to execute command");

            let res = output.status.success();
            println!("is_wsl: {}", res);
            res
        }
        _ => false,
    }
}

pub fn split_args(command: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut parts = command.split_whitespace();
    while let Some(part) = parts.next() {
        args.push(part.to_string());
    }
    args
}

pub fn run_command_common<P: AsRef<Path>>(
    current_dir: P,
    program: &str,
    args: &Vec<String>,
    name: &str,
    product_id: &str,
) -> Result<(), String> {
    let child = Command::new(program)
        .current_dir(current_dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let child = Arc::new(Mutex::new(child));
    let work_thread = thread::Builder::new()
        .name(name.to_string())
        .stack_size(1024 * 4);
    let child_spawn = child.clone();
    let name = name.to_string();
    let _handle = work_thread.spawn(move || {
        if let Some(stdout) = child_spawn.lock().unwrap().stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("Output: {}", line.expect("Failed to read line"));
                if let Ok(Some(status)) = child_spawn.lock().unwrap().try_wait() {
                    println!("child[{}] exit status: {:?}", name, status);
                    break;
                }
            }
        }
    });

    let sub_process_id = child.lock().unwrap().id();
    println!("sub_process_id:{}", sub_process_id);

    APP_INSTALLED
        .lock()
        .unwrap()
        .insert(product_id.to_string(), Some(child));
    Ok(())
}

pub fn get_file_name_without_suffix(file_path: &str) -> String {
    let name = Path::new(file_path).file_stem();
    match name {
        Some(name) => name.to_string_lossy().to_string(),
        None => file_path.to_string(),
    }
}
