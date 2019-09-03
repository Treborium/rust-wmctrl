use crate::transformation::Transformation;
use crate::state::State;

use super::wmctrl;

pub struct Window {
    id: String,
    desktop: String,
    client_machine: String,
    title: String,
    transformation: Transformation,
}

impl Window {
    pub(super) fn new(id: String, desktop: String, client_machine: String, title: String, transformation: Transformation) -> Window {
        Window { id, desktop, client_machine, title, transformation }
    }

    pub(super) fn get(&self) -> String {
        format!("{} -i", self.id)
    }

    pub fn set_long_title(&mut self, title: &str) {
        self.title = String::from(title);

        let args = format!("-r {} -N {}", self.get(), title);
        wmctrl(&args);
    }

    pub fn set_short_title(&self, title: &str) {
        let args = format!("-r {} -I {}", self.get(), title);
        wmctrl(&args);
    }

    pub fn set_both_title(&mut self, title: &str) {
        self.title = String::from(title);

        let args = format!("-r {} -T {}", self.get(), title);
        wmctrl(&args);

    }

    pub fn change_state(&self, state: State) {
        let args = format!("-r {} -b {}", self.get(), state);
        wmctrl(&args);
    }

    pub fn transform(&mut self, transformation: Transformation) {
        self.transformation = transformation;

        let args = format!("-r {} -e {}", self.get(), &self.transformation);
        wmctrl(&args);
    }

    pub fn set_desktop(&mut self, desktop: &str) {
        self.desktop = String::from(desktop);

        let args = format!("-r {} -t {}", self.get(), desktop);
        wmctrl(&args);
    }

    /// Moves the window to the current desktop and raises it
    pub fn activate(&mut self) {
        self.desktop = super::get_current_desktop();

        let args = format!("-R {}", self.get());
        wmctrl(&args);
    }

    pub fn raise(&self) {
        let args = format!("-a {}", self.get());
        wmctrl(&args);
    }

    pub fn close(&self) {
        let args = format!("-c {}", self.get());
        wmctrl(&args);
    }
}