fn task1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

/*fn task2() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n as f64 / 2.0
        };

    println!("{} -> {}", n, big_n);
}*/

fn task3() {
    for n in 1..=99 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

fn task4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in &numbers {
    }

    println!("{:?}", numbers);
}

fn task5() {
    let a = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

fn task6() {
    let mut n = 1;

    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

fn task7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

fn task8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

fn task9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    assert_eq!(count, 5);
    println!("Success!");
}

fn task10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("Success!");
}

fn main() {
    task1();
/*    task2();*/
    task3();
    task4();
    task5();
    task6();
    task7();
    task8();
    task9();
    task10();
}
