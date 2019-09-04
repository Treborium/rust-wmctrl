use std::process::Command;
use std::process::Output;

use crate::window::Window;

// This function is only visible crate internally
pub(crate) fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

/// Get the currently active Desktop
pub fn get_current_desktop() -> String {
    let output = String::from_utf8(super::list_desktops().stdout).unwrap();

    let columns = output
        .lines()
        .find(|line| line.contains("*"))
        .unwrap()
        .split(" ")
        .filter(|column| !column.is_empty())
        .collect::<Vec<&str>>();

    String::from(columns[0])
}

/// Find a window by title inside a Vector
///
/// This method is case insensitive
pub fn find_window_by_title<'a>(windows: &'a Vec<Window>, title: &str) -> Option<&'a Window> {
    windows.into_iter().find(|w| {
        w.title()
            .to_lowercase()
            .contains(title.to_lowercase().as_str())
    })
}
