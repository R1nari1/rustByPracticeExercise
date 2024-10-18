use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn task1() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            println!("Success!");
        } else {
            println!("Exercise Failed if printing out this line!");
        }
    }

    drink("lemonade");
    println!("Exercise Failed if printing out this line!");
}

fn task2() {
    fn divide(x: u8, y: u8) {
        if y == 0 {
            println!("Division by zero is not allowed!");
        } else {
            println!("{}", x / y);
        }
    }

    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 221;
        match speed {
            1..=4 => (speed * cph) as f64,
            5..=8 => (speed * cph) as f64 * 0.9,
            9..=10 => (speed * cph) as f64 * 0.77,
            _ => 0 as f64,
        }
    }

    assert_eq!("abc".as_bytes(), [97, 98, 99]); // Исправленное сравнение

    let v = vec![1, 2, 3];
    if let Some(ele) = v.get(2) {
        println!("Element: {}", ele);
    } else {
        println!("No element found.");
    }

    divide(15, 1);

    println!("Success!");
}

fn task3() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    assert_eq!(multiply("10", "2").unwrap(), 20);
    assert!(multiply("t", "2").is_err());

    println!("Success!");
}

fn task4() {
    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}

fn task5() {
    type Res<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Res<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}

