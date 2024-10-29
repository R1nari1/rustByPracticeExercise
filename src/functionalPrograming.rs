fn task1() {
    let color = String::from("green");

    let print = move || println!("`color`: {}", color);

    print();
    print();

    let _reborrow = &color;

    println!("{}",color);
}

fn task2() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    let _reborrow = &count;

    inc();

    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}

fn task3() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable);
    };

    consume();
    consume();
}

fn take<T>(_v: T) {}


fn task4() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}

fn task5_fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn task5() {
    let x = vec![1, 2, 3];
    task5_fn_once(|z| z == x.len())
}

fn task6() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}


fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}


fn task7() {
    let mut s = String::new();

    let update_string = |str| -> String { s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}


fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn task8() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;
    move |x| x + num
}

fn task9() {
    let fn_plain = create_fn();
    fn_plain(1);
}

fn factory(x: i32) -> impl Fn(i32) -> i32 {
    let num = 5;

    if x > 1 {
        move |x| x + num
    } else {
        move |x| x + num
    }
}

fn task10() {
    let arr = [0; 10];
    for item in arr.iter() {
        println!("{}", item);
    }
}

fn task11() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

fn task12() {
    let mut v1 = vec![1, 2].into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}

fn task13() {
    let arr = vec![0; 10];
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("{:?}", arr);
}

fn task14() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn task15() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}

fn task16() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}", v1);
}

use std::collections::HashMap;

fn task17() {
    let names = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().cloned().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}

fn task18() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

/* task19 - Fill in the blanks */
use std::collections::HashMap;

fn task19() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}", folks);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn task20() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

