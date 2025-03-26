use std::{
    path::Path,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
};

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
    _name: &str,
    _pid: &str,
) -> Result<Arc<Mutex<Child>>, String> {
    let current_dir = current_dir
        .as_ref()
        .canonicalize()
        .map_err(|e| e.to_string())?;

    let child = Command::new(program)
        .current_dir(current_dir)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| e.to_string())?;

    let sub_process_id = child.id();
    println!("sub_process_id:{}", sub_process_id);

    let child = Arc::new(Mutex::new(child));

    Ok(child)
}

pub fn get_file_name_without_suffix(file_path: &str) -> String {
    let name = Path::new(file_path).file_stem();
    match name {
        Some(name) => name.to_string_lossy().to_string(),
        None => file_path.to_string(),
    }
}
