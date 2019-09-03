use std::process::Command;

pub mod transformation;
pub mod state;


pub fn help() -> std::process::Output {
    wmctrl("-h")
}

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

/// window: the window id or a string that matches part of the title
pub fn activate_window(window: &str) -> std::process::Output {
    let args = format!("-a {}", parse_window(window));
    wmctrl(&args)
}

pub fn close_window(window: &str) -> std::process::Output {
    let args = format!("-c {}", parse_window(window));
    wmctrl(&args)
}

/// Moves the window to the current desktop and raises it
pub fn move_window_to_current_desktop(window: &str) -> std::process::Output {
    let args = format!("-R {}", parse_window(window));
    wmctrl(&args)
}

/// Moves the window to the specified desktop
pub fn move_window(window: &str, desktop: &str) -> std::process::Output {
    let args = format!("-r {} -t {}", parse_window(window), desktop);
    wmctrl(&args)
}

pub fn move_and_resize(window: &str, transformation: transformation::Transformation) -> std::process::Output {
    let args = format!("-r {} -e {}", parse_window(window), transformation);
    wmctrl(&args)
}

pub fn change_state(window: &str, state: state::State) -> std::process::Output {
    let args = format!("-r {} -b {}", parse_window(window), state);
    wmctrl(&args)
}

fn wmctrl(args: &str) -> std::process::Output {
     Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

fn parse_window(window: &str) -> String {
    if window.starts_with("0x") {
        format!("{} -i", window)
    } else {
        window.to_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
