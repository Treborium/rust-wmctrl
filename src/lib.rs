use std::process::Command;
use std::str;

pub fn print_windows() -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg("wmctrl -l")
        .output()
        .expect("failed to execute 'wmctrl -l'")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
