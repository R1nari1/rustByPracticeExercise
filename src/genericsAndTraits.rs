// Task 1
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn task1() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}

// Task 2
struct Point<T> {
    x: T,
    y: T,
}

fn task2() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

// Task 3
struct Point2<T, U> {
    x: T,
    y: U,
}

fn task3() {
    let p = Point2 { x: 5, y: "hello".to_string() };

    println!("Success!");
}

// Task 4
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn task4() {
    let x = Val { val: 3.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());
}

// Task 5
struct Point3<T> {
    x: T,
    y: T,
}

impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn task5() {
    let p = Point3 { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin());
}


//Const Generics

struct Array<T, const N: usize> {
    data: [T; N]
}

fn task6() {
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];

    println!("Success!");
}

fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn task7() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

//Traits

trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn task8() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}

//Array with trait objects

trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn task14() {
    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
    }
}

//

/*use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn task15() {
    assert_eq!(
        Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 }
    );

    println!("Success!");
}
*/
