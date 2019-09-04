use std::fmt;

/// Actions that may be applied to a `wmctrl::Window`
pub enum Action {
    Remove,
    Add,
    Toggle,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Remove => write!(f, "remove"),
            Action::Add => write!(f, "add"),
            Action::Toggle => write!(f, "toggle"),
        }
    }
}

/// Properties that may be applied to a `wmctrl::Window`
pub enum Property {
    Modal,
    Sticky,
    MaximizedVert,
    MaximizedHorz,
    Shaded,
    SkipTaskbar,
    SkipPager,
    Hidden,
    Fullscreen,
    Above,
    Below,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Property::Modal => write!(f, "modal"),
            Property::Sticky => write!(f, "sticky"),
            Property::MaximizedVert => write!(f, "maximized_vert"),
            Property::MaximizedHorz => write!(f, "maximized_horz"),
            Property::Shaded => write!(f, "shaded"),
            Property::SkipTaskbar => write!(f, "skip_taskbar"),
            Property::SkipPager => write!(f, "skip_pager"),
            Property::Hidden => write!(f, "hidden"),
            Property::Fullscreen => write!(f, "fullscreen"),
            Property::Above => write!(f, "above"),
            Property::Below => write!(f, "below"),
        }
    }
}

/// Holds information about the new state of a `wmctrl::Window`.
pub struct State {
    action: Action,
    property: Property,
}

impl State {
    pub fn new(action: Action, property: Property) -> State {
        State { action, property }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.action, self.property)
    }
}
