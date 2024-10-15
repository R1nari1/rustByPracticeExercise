//string
fn task1() {
    let s: &str = "hello, world";
    println!("Success!");
}

fn task2() {
    let s: &str = "hello, world";
    println!("Success!");
}

fn task3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn task4() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

fn task5() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

fn task6() {
    let s = String::from("hello, world");
    greetings(s.clone());
}

fn greetings(s: String) {
    println!("{}", s)
}

fn task7() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

fn task8() {
    let byte_escape = "I'm writing Ru\x73st!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

fn task9() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

//array

fn task10() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    println!("Success!");
}

fn task11() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);
    println!("Success!");
}

fn task12() {
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("Success!");
}

fn task13() {
    let _arr = [1, 2, 51];
    println!("Success!");
}

fn task14() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!(ele == 'a');
    println!("Success!");
}

fn task15() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let name0 = names.get(0).unwrap();
    println!("Success!");
}

//slice
fn task16() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}

fn task17() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

fn task18() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[0..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

fn task19() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}

/*fn task20() {
    let mut s = String::from("hello world");

    let letter = first_letter(&s);

    s.clear();

    println!("the first letter is: {}", letter);
}*/

fn first_letter(s: &str) -> &str {
    &s[..1]
}

//Tuple

fn task21() {
    let _t0: (u8, i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

fn task22() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}

/*fn task23() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}*/

fn task24() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

fn task25() {
    let (x, y, z);

    (x, y, z) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

fn task26() {
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
//Struct

// Task 27
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn task27() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}

// Task 28
struct Unit;

trait SomeTrait {}

impl SomeTrait for Unit {}

fn task28() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

fn do_something_with_unit(u: Unit) {}

// Task 29
struct Person2 {
    name: String,
    age: u8,
}

fn task29() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;

    p.name = String::from("sunfei");

    println!("Success!");
}

// Task 30
struct Person3 {
    name: String,
    age: u8,
}

fn task30() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name,
    }
}

// Task 31
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn task31() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: u.username,
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}

// Task 32
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn task32() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{:?}", rect1);
}

// Task 33
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn task33() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name.clone();

    println!("{}, {}, {:?}", _name, f.data, f);
}

