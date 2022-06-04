//! Desktop related functions.

use std::process::Output;

use super::wmctrl;

/// List desktops. The current desktop is marked with an asterisk
///
/// This function is the equivalent of `wmctrl -d`.
///
/// # Examples
///
/// ```
/// use wmctrl::desktop;
///
/// println!("{}", String::from_utf8(desktop::list_desktops().stdout).unwrap());
/// ```
pub fn list_desktops() -> Output {
    wmctrl("-d")
}

/// Switch to the specified desktop
///
/// This function is the equivalent of `wmctrl -s <DESK>`.
///
/// # Examples
///
/// ```
/// use wmctrl::desktop;
///
/// desktop::switch_desktop("1");
/// ```
pub fn switch_desktop(desktop: &str) -> Output {
    wmctrl(&format!("-s {}", desktop))
}

/// Change the number of desktops
///
/// This function is the equivalent of `wmctrl -n <NUM>`.
pub fn set_desktop_count(count: u8) -> Output {
    wmctrl(&format!("-n {}", count))
}

/// Get the currently active Desktop
pub fn get_current_desktop() -> i32 {
    let output = String::from_utf8(list_desktops().stdout).unwrap();

    let columns = output
        .lines()
        .find(|line| line.contains('*'))
        .unwrap()
        .split(' ')
        .filter(|column| !column.is_empty())
        .collect::<Vec<&str>>();

    // Parse the string to an integer.
    String::from(columns[0]).parse().unwrap()
}

// Get the name of the currently active Desktop
pub fn get_current_desktop_name() -> String {
    let output = String::from_utf8(list_desktops().stdout).unwrap();

    let columns = output
        .lines()
        .find(|line| line.contains('*'))
        .unwrap()
        .split(' ')
        .filter(|column| !column.is_empty())
        .collect::<Vec<&str>>();

    String::from(columns[14])
}
