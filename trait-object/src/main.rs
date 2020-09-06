fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button { text: String::from("button1") }),
            Box::new(Button { text: String::from("button2") }),
            Box::new(Text { text: String::from("text1") }),
            Box::new(Text { text: String::from("text2") }),
        ]
    };
    s.run();
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

struct Button {
    text: String,
}

struct Text {
    text: String,
}

impl Screen {
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.text);
    }
}

impl Draw for Text {
    fn draw(&self) {
        println!("Text: {}", self.text);
    }
}