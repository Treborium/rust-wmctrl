/// A wrapper for the `wmctrl` command line interface.
///
/// # Examples
///
/// Move firefox to desktop 2 and make it fullscreen
/// ```
/// let windows = get_windows();
/// let mut firefox = utils::find_window_by_title(&windows, "firefox").unwrap();
/// 
/// firefox.set_desktop("1");
/// firefox.change_state(State::new(Action::Add, Property::Fullscreen));
/// ```
use std::process::Output;

pub mod desktop;
pub mod utils;

mod transformation;
mod window;
mod state;

pub use transformation::Transformation;
pub use window::Window;
pub use state::State;
pub use state::Action;
pub use state::Property;

use utils::wmctrl;

/// Print help
///
/// This function is the equivalent of `wmctrl -h`.
/// # Examples
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
/// 
/// You should never fetch a second `Vec<Window>` while the first one is still in use!
/// This will lead to inconsistencies between the two Vectors and most likely to incorrect behavior of your code.
/// It is advised to get a `Vec<Window>` once at the beginning of the program and operate on it.
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

fn parse_row(row: &str) -> Window {
    let columns = row.split(" ")
        .filter(|e| !e.is_empty())
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
