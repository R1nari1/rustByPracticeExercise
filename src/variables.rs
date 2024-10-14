pub fn task1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

pub fn task2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

pub fn task3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
        println!("The value of x is {} and value of y is {}", x, y);
    }
}

pub fn task4() {
    let x = "hello";
    println!("{}, world", x);
}