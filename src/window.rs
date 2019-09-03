use crate::transformation::Transformation;

pub struct Window {
    pub id: Option<String>,
    pub desktop: String,
    pub client_machine: String,
    pub title: String,
    pub transformation: Transformation,
}

impl Window {
    pub fn new(id: Option<String>, desktop: String, client_machine: String, title: String, transformation: Transformation) -> Window {
        Window { id, desktop, client_machine, title, transformation }
    }

    pub fn get(&self) -> String {
        match &self.id {
            Some(id) => format!("{} -i", id).clone(),
            None => self.title.clone(),
        }
    }
}