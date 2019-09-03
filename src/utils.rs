use std::process::Command;
use std::process::Output;

pub(crate) fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

pub fn get_current_desktop() -> String {
    // TODO: Implement me
    String::from("Method not implemented!")
}