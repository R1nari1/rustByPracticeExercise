fn task1() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

fn task2() {
    /* Fill in the blanks to make it print:
    Hello world, I am
    Sunface!
    */
    println!("Hello world, ");
    println!("I am");
    println!("Sunface!");
}

/* Fill in the blanks and Fix the errors */
struct Structure(i32);

fn task3() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn task4() {
    let person = Person {
        name: "Sunface".to_string(),
        age: 18,
    };

    /* Make it output:
    Person {
        name: "Sunface",
        age: 18,
    }
    */
    println!("{:?}", person);
}

#[derive(Debug)]
struct Structure(i32);

struct Deep(Structure);

impl std::fmt::Debug for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (self.0).0)
    }
}

fn task5() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}


use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn task6() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");

    println!("Success!");
}

fn task7() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21"); // Fill: "21"
    assert_eq!(format!("{1}{0}", 2, 1), "2112"); // Fill: "2112"
    println!("Success!");
}

fn task8() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

    /* Fix the error */

    println!("{abc} {}", 2, abc = "def");

    println!("Success!");
}


fn task9() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);

    /* Fill in the blanks */
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");

    println!("Success!");
}

fn task10() {
    println!("Hello {:5}!", 5);
    println!("Hello {:+}!", 5);
    println!("Hello {:05}!", 5);
    println!("Hello {:05}!", -5);

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

    println!("Success!");
}

/* Fill in the blanks */
fn task11() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4);

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!");
}
