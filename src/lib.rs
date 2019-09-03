use std::process::Command;
use std::process::Output;


// pub mod transformation;
pub mod transformation;
pub mod state;
pub mod window;

pub use transformation::Transformation;
pub use state::State;
pub use window::Window;


pub fn help() -> Output {
    wmctrl("-h")
}

pub fn list_windows() -> Vec<Window> {
    let output_table = String::from_utf8(wmctrl("-l -G").stdout)
        .unwrap();

    let mut windows = Vec::new();
    for row in output_table.lines() {
        windows.push(parse_row(row))
    }

    windows
}

/// This equals the -m flag
pub fn show_wm_information() -> Output {
    wmctrl("-m")
}

/// This equals the -d flag
pub fn list_desktops() -> Output {
    wmctrl("-d")
}

pub fn get_current_desktop() -> String {
    // TODO: Implement me
    String::from("Method not implemented!")
}

/// This equals the -s flag
/// desktop usually means workspace in this context
pub fn switch_desktop(desktop: &str) -> Output {
    wmctrl(&format!("-s {}", desktop))
}

pub fn set_desktop_count(count: u8) -> Output {
    wmctrl(&format!("-n {}", count))
}

pub fn wmctrl(args: &str) -> Output {
     Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .expect(&format!("failed to execute 'wmctrl {}'", args))
}

fn parse_row(row: &str) -> Window {
    let columns = row.split(" ").collect::<Vec<&str>>();

    // Filter empty strings out
    let columns = columns.into_iter()
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

    title_substrings.into_iter()
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
