use wmctrl::*;

fn main() {
    let windows = wmctrl::get_windows();
    let win = wmctrl::utils::find_window_by_title(&windows, "brave");
    let desktops = desktop::list_desktops();

    println!("Full title: {}", win.unwrap().title());

}