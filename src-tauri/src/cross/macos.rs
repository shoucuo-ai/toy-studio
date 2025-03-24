#[cfg(target_os = "macos")]
pub fn run_command<P: AsRef<std::path::Path>>(
    current_dir: P,
    program: &str,
    args: &Vec<String>,
    name: &str,
    product_id: &str,
) -> Result<std::sync::Arc<std::sync::Mutex<std::process::Child>>, String> {
    crate::run_command_common(current_dir, program, args, name, product_id)
}
