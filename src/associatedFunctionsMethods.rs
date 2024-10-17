// Task 1
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/*fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
    println!("Success!");
}*/

// Task 2
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}

/*fn main() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);
}*/

// Task 3
struct TrafficLight2 {
    color: String,
}

impl TrafficLight2 {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

/*fn main() {
    let mut light = TrafficLight { color: "red".to_string() };
    light.show_state();
    light.change_state();
    light.show_state();
    println!("Success!");
}*/

// Task 4
#[derive(Debug)]
struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

/*fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    println!("Success!");
}*/

// Task 5
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle2 {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    assert!(rect1.can_hold(&rect2));
    println!("Success!");
}*/