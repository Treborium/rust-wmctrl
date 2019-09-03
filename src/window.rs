#[derive(Debug)]
pub struct Window {
    pub id: String,
    pub desktop: String,
    pub client_machine: String,
    pub title: String,
}

impl Window {
    pub fn new(id: String, desktop: String, client_machine: String, title: String) -> Window {
        Window { id, desktop, client_machine, title }
    }

    pub fn get(&self) -> String {
        format!("{} -i", self.id)
    }
}