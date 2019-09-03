use wmctrl::print_windows;


fn main() {
    let output = wmctrl::print_windows();
    println!("wmctrl says: {}", 
        String::from_utf8(output.stdout)
        .expect("Could not parse the output of the command")
    );
}