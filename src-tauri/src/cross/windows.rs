#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
pub fn run_command<P: AsRef<Path>>(
    current_dir: P,
    program: &str,
    args: &Vec<String>,
    name: &str,
    product_id: &str,
) -> Result<(), String> {
    let template = r#"cd {} && {}"#;
    let cmd = format!(
        template,
        current_dir.as_ref().to_string_lossy(),
        program,
        args.join(" "),
    );
    println!("cmd:{}", cmd);

    use std::os::windows::process::CommandExt;
    const CREATE_NEW_CONSOLE: u32 = 0x00000010;
    Command::new("cmd.exe")
        .current_dir(current_dir)
        .args(&["/C", &cmd])
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .expect("无法启动cmd进程");
    Ok(())
}
