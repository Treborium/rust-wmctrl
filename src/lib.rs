use std::process::Command;

pub fn list_windows() -> std::process::Output {
   wmctrl("-l")
}

/// This equals the -m flag
pub fn show_wm_information() -> std::process::Output {
    wmctrl("-m")
}

/// This equals the -d flag
pub fn list_desktops() -> std::process::Output {
    wmctrl("-d")
}

/// This equals the -s flag
/// desktop usually means workspace in this context
pub fn switch_desktop(desktop: &str) -> std::process::Output {
    wmctrl(&format!("-s {}", desktop))
}

fn wmctrl(args: &str) -> std::process::Output {
     Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
