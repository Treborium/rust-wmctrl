use crate::transformation::Transformation;

pub struct Window {
    pub id: String,
    pub desktop: String,
    pub client_machine: String,
    pub title: String,
    pub transformation: Transformation,
}

impl Window {
    pub(super) fn new(id: String, desktop: String, client_machine: String, title: String, transformation: Transformation) -> Window {
        Window { id, desktop, client_machine, title, transformation }
    }

    pub fn get(&self) -> String {
        format!("{} -i", self.id)
    }
}