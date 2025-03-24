#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
pub fn run_command<P: AsRef<std::path::Path>>(
    current_dir: P,
    program: &str,
    args: &Vec<String>,
    _name: &str,
    _pid: &str,
) -> Result<std::sync::Arc<std::sync::Mutex<std::process::Child>>, String> {
    match current_dir.as_ref().canonicalize() {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(current_dir) => {
            let abs_dir = current_dir.to_string_lossy().to_string();
            // 去掉 `\\?\` 前缀
            let abs_dir = if abs_dir.starts_with(r"\\?\") {
                &abs_dir[4..]
            } else {
                &abs_dir
            };
            println!("abs_dir:{}", abs_dir);
            let args = args.join(" ");
            let cmd = format!("cd {abs_dir} && {program} {args}");
            // 添加pause防止窗口自动关闭
            let cmd = if !cmd.contains("pause") {
                format!("{cmd} && pause")
            } else {
                cmd
            };
            println!("cmd:{}", cmd);

            use std::os::windows::process::CommandExt;

            // 添加@ECHO ON，确保显示将要执行的每个命令
            // let echo_cmd = format!("@ECHO ON && {cmd}");
            let tip = cmd.replace("\"", "\\\"");
            let echo_cmd = format!(r#"echo "Executing: {tip}" && {cmd}"#);

            let child = std::process::Command::new("cmd.exe")
                .current_dir(std::path::Path::new(abs_dir))
                .args(&["/V:ON", "/K", &echo_cmd])
                .creation_flags(0x00000010) // 创建新控制台 CREATE_NEW_CONSOLE
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("无法启动cmd进程");
            Ok(std::sync::Arc::new(std::sync::Mutex::new(child)))
        }
    }
}
