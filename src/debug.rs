pub struct Debugger {
    data: String
}

impl Debugger { 
    pub fn new() -> Self {
        Debugger {
            data: String::new(),
        }
    }

    pub fn update(&mut self, data: String) {
        self.data = data;
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}