enum Direction {
    East,
    West,
    North,
    South,
}

fn task1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

fn task2() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn task3() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants"),
    }
}

fn task4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z'));
    }

    println!("Success!");
}

enum MyEnum {
    Foo,
    Bar,
}

fn task5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if let MyEnum::Foo = e {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

fn task6() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }

    println!("Success!");
}

enum Foo {
    Bar(u8),
}

fn task7() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

enum Foo2 {
    Bar,
    Baz,
    Qux(u32),
}

fn task8() {
    let a = Foo2::Qux(10);

    match a {
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo::baz"),
        Foo2::Qux(_) => println!("match others"),
    }
}

fn task9() {
    let age = Some(30);
    if let Some(age_value) = age {
        assert_eq!(age_value, 30);
    }

    match age {
        Some(age_value) => println!("age is a new variable, its value is {}", age_value),
        _ => (),
    }
}

//pattern

fn task10() {
    fn match_number(n: i32) {
        match n {
            1 => println!("One!"),
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            6..=10 => println!("match 6 -> 10"),
            _ => println!("match -infinite -> 0 or 11 -> +infinite"),
        }
    }

    match_number(3);
    match_number(7);
}

fn task11() {
    let num = Some(4);
    let split = 5;

    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

fn task12() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

fn task13() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }

    println!("{}", v);  
}
