use std::process::Command;
use std::process::Output;

use crate::window::Window;

pub(crate) fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

pub fn get_current_desktop() -> String {
    let output = String::from_utf8(super::list_desktops().stdout).unwrap();

    let columns = output.lines()
        .find(|line| line.contains("*"))
        .unwrap()
        .split(" ")
        .filter(|column| !column.is_empty())
        .collect::<Vec<&str>>();

    String::from(columns[0])
}

pub fn find_window_by_title(title: &str) -> Option<Window> {
    super::get_windows()
        .into_iter()
        .find(|w| w.title().contains(title))
}
