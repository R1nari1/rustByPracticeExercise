//numbers
pub fn task1() {
    let x: i32 = 5;
    let mut y: u32 = 5;
    y = x as u32;

    let z: i32 = 10; // Задаем тип переменной z

    println!("Success!");
}

pub fn task2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

pub fn task3() {

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

/*pub fn task4() {
    let v1 = 251_u8 + 8; // переповнення
    let v2 = i8::checked_add(120, 8).unwrap();
    println!("{},{}",v1,v2);
}*/

pub fn task5() {
    let x = 1_000.000_1_f64;
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string()); // Тип x — f64
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn task6() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5); // Исправляем значение суммы, правильное значение будет -5

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

pub fn task7() {

    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);

    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    /*assert!((9.6 / 3.2 - 3.0).abs() < f64::EPSILON);*/

    assert!(24 % 5 == 4);


    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);


    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

//Char, Bool and Unit

fn task8() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

fn task9() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

fn task10() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

fn task11() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

fn task12() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2, 3));

    println!("Success!");
}

//Statements and Expressions

fn task13() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
    println!("Success!");
}

fn task14() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
    println!("Success!");
}

fn task15() {
    let s = sum(1, 2);
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

//function

fn task16() {
    print();
}

fn print() -> () {
    println!("Success!");
}
