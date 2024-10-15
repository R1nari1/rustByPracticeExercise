fn task1() {
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}", x, y);
}

fn task2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn task3() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s = s.clone().into_bytes();
    String::from_utf8(_s).unwrap()
}

fn task4() {
    let s = String::from("Hello World");
    print_str(s.clone());
    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s);
}

fn task5() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

fn task6() {
    let mut s1 = String::from("Hello ");
    s1.push_str("World!");
    println!("Success!");
}

fn task7() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0.clone();
    println!("{:?}", t);
}

fn task8() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = (t.0.clone(), t.1.clone());
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}

//Reference and Borrowing

fn task15() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");

    // Коментую щоб не було помилок
    // let r2 = &mut s;
    // r2.push_str("!");

    println!("{}", r1);
}

fn task16() {
    let x = 5;
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn task17() {
    let x = 5;
    let y = &x;

    assert_eq!(*y, 5);

    println!("Success!");
}

fn task18() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {
    println!("{}", s);
}

fn task19() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world");
}

fn task20() {
    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

fn task21() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", r1);

    println!("Success!");
}

fn task22() {
    let mut s = String::from("hello, ");

    borrow_object_mut(&mut s);

    println!("Success!");
}

fn borrow_object_mut(s: &mut String) {
    s.push_str("world!");
}

fn task23() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn task24() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");

    // let r2 = &mut s;
    // r2.push_str("!");

    println!("{}", r1);
}

