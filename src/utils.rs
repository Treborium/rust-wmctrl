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
