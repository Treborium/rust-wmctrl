use std::process::Command;
use std::process::Output;

use crate::window::Window;

// This function is only visible crate internally
pub(crate) fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .unwrap_or_else(|_| panic!("failed to execute 'wmctrl {}'", args))
}

/// Find a window by title inside a Vector and return a reference of the entry
///
/// This method is case insensitive
pub fn find_window_by_title<'a>(windows: &'a [Window], title: &str) -> Option<&'a Window> {
    windows.iter().find(|w| {
        w.title()
            .to_lowercase()
            .contains(title.to_lowercase().as_str())
    })
}

/// Find a window by title inside a Vector and return a mutable reference of the entry
///
/// This method is case insensitive
pub fn find_window_by_title_mut<'a>(
    windows: &'a mut Vec<Window>,
    title: &str,
) -> Option<&'a mut Window> {
    windows.iter_mut().find(|w| {
        w.title()
            .to_lowercase()
            .contains(title.to_lowercase().as_str())
    })
}
