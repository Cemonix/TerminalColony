pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}