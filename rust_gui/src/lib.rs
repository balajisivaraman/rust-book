pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // The Box<Draw> is s trait object here
    // it's a stand-in for any type inside a Box that implements the Draw trait
    pub components: Vec<Box<Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code goes here
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
