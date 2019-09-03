use std::process::Command;

pub mod transformation;
pub mod state;
pub mod window;


pub fn help() -> std::process::Output {
    wmctrl("-h")
}

pub fn list_windows() -> Vec<window::Window> {
    let output_table = String::from_utf8(wmctrl("-l").stdout)
        .unwrap();

    let mut windows = Vec::new();

    for row in output_table.lines() {
        let mut columns: Vec<&str> = row.split(" ").collect();

        if columns[3] == " " {
            columns.remove(3);
        }

        let title_substrings = columns[3..].to_vec();

        let mut title = String::from("");
        title_substrings.iter().for_each(|&e| title += e);

        windows.push(window::Window::new(String::from(columns[0]),
            String::from(columns[1]),
            String::from(columns[2]),
            title
        ));
    }

    windows
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
pub fn activate_window(window: &window::Window) -> std::process::Output {
    let args = format!("-a {}", window.get());
    wmctrl(&args)
}

pub fn close_window(window: &window::Window) -> std::process::Output {
    let args = format!("-c {}", window.get());
    wmctrl(&args)
}

/// Moves the window to the current desktop and raises it
pub fn move_window_to_current_desktop(window: &window::Window) -> std::process::Output {
    let args = format!("-R {}", window.get());
    wmctrl(&args)
}

/// Moves the window to the specified desktop
pub fn move_window(window: &window::Window, desktop: &str) -> std::process::Output {
    let args = format!("-r {} -t {}", window.get(), desktop);
    wmctrl(&args)
}

pub fn move_and_resize(window: &window::Window, transformation: transformation::Transformation) -> std::process::Output {
    let args = format!("-r {} -e {}", window.get(), transformation);
    wmctrl(&args)
}

pub fn change_state(window: &window::Window, state: state::State) -> std::process::Output {
    let args = format!("-r {} -b {}", window.get(), state);
    wmctrl(&args)
}

pub fn set_long_title(window: &window::Window, title: &str) -> std::process::Output {
    let args = format!("-r {} -N {}", window.get(), title);
    wmctrl(&args)
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
