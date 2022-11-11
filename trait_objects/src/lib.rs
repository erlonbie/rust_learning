pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen{
   pub fn run(&self) {
       for component in self.components.iter() {
           component.draw();
       }
   }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// impl Button {
//     pub fn new(width: u32, height: u32, label: String) -> Self { Self { width, height, label } }
// }

impl Draw for Button {
    fn draw(&self) {
       //draw button 
    }    
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T: Draw> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
