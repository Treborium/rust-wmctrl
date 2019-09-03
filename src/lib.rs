use std::process::Command;

pub fn list_windows() -> std::process::Output {
   wmctrl("-l")
}

/// This equals the -m flag
pub fn show_wm_information() -> std::process::Output {
    wmctrl("-m")
}

/// This equald the -d flag
pub fn list_desktops() -> std::process::Output {
    wmctrl("-d")
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
