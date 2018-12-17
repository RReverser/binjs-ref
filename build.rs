use std::process::*;

const PATH_PACKAGES: &'static str = "package.json";

fn exec(cmd: &str) -> Result<(), ()> {
    eprintln!("Bootstrap: Attempting to execute `{}`", cmd);

    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/c", cmd]).status()
    } else {
        Command::new("sh").args(&["-c", cmd]).status()
    };

    match status {
        Ok(status) if status.success() => Ok(()),
        // error details are already printed to stdout/stderr, so just () is fine
        _ => Err(()),
    }
}

fn main() -> Result<(), ()> {
    // Install Node packages if necessary.
    println!("cargo:rerun-if-changed={}", PATH_PACKAGES);

    exec("yarn install").or_else(|()| exec("npm install"))
}
