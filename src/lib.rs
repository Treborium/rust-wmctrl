/// A wrapper for the `wmctrl` command line interface.
///
/// # Examples
///
/// Move firefox to desktop 2 and make it fullscreen
/// ```
/// let mut win: Window = wmctrl::get_windows().into_iter().find(|w| w.title().contains("firefox")).unwrap();
/// win.set_desktop("1");
/// win.change_state(State::new(state::Action::Add, state::Property::Fullscreen));
/// ```
use std::process::Command;
use std::process::Output;

pub mod state;
pub mod transformation;
pub mod window;

pub use state::State;
pub use transformation::Transformation;
pub use window::Window;

/// Print help
///
/// This function is the equivalent of `wmctrl -h`.
/// /// # Examples
///
/// ```
/// println!("{}", String::from_utf8(wmctrl::help().stdout).unwrap());
/// ```
pub fn help() -> Output {
    wmctrl("-h")
}

/// Get windows managed by the window manager
///
/// This function is the equivalent of `wmctrl -l -G`.
pub fn get_windows() -> Vec<Window> {
    let output_table = String::from_utf8(wmctrl("-l -G").stdout).unwrap();

    let mut windows = Vec::new();
    for row in output_table.lines() {
        windows.push(parse_row(row))
    }

    windows
}

/// Show information about the window manager and about the environment
///
/// This function is the equivalent of `wmctrl -m`.
///
/// # Examples
///
/// ```
/// println!("{}", String::from_utf8(wmctrl::show_wm_information().stdout).unwrap());
/// ```
pub fn show_wm_information() -> Output {
    wmctrl("-m")
}

/// List desktops. The current desktop is marked with an asterisk
///
/// This function is the equivalent of `wmctrl -d`.
///
/// # Examples
///
/// ```
/// println!("{}", String::from_utf8(wmctrl::list_desktops().stdout).unwrap());
/// ```
pub fn list_desktops() -> Output {
    wmctrl("-d")
}

pub fn get_current_desktop() -> String {
    // TODO: Implement me
    String::from("Method not implemented!")
}

/// Switch to the specified desktop
///
/// This function is the equivalent of `wmctrl -s <DESK>`.
///
/// # Examples
///
/// ```
/// wmctrl::switch_desktop("1");
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

pub(crate) fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

fn parse_row(row: &str) -> Window {
    let columns = row.split(" ").collect::<Vec<&str>>();

    // Filter empty strings out
    let columns = columns
        .into_iter()
        .filter(|&e| !e.is_empty())
        .collect::<Vec<&str>>();

    let (x, y, w, h) = (
        columns[2].parse::<u16>().unwrap(),
        columns[3].parse::<u16>().unwrap(),
        columns[4].parse::<u16>().unwrap(),
        columns[5].parse::<u16>().unwrap(),
    );

    let t = Transformation::new(x, y, w, h);

    let (id, desktop, client_machine) = (
        columns[0].to_owned(),
        columns[1].to_owned(),
        columns[6].to_owned(),
    );

    let mut title = String::from("");
    let title_substrings: Vec<&str> = columns[7..].to_vec();

    title_substrings
        .into_iter()
        .for_each(|e| title += format!("{} ", e).as_str());
    // Remove last whitespace
    title.pop();

    Window::new(id, desktop, client_machine, title, t)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
