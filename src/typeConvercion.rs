fn task1() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Integer as char: {}", c2);
    println!("Success!");
}


fn task2() {
    assert_eq!(u8::MAX, 255);

    let v = 1000 % 256;

    println!("Success!");
}

/*fn task3() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
*/
fn task4() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(num: i32) -> Self {
            Number { value: num }
        }
    }

    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}

fn task5() {
    let n: i16 = 256;

    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}

fn task6() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}

fn task7() {
    use std::str::FromStr;
    use std::num::ParseIntError;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32
    }

    impl FromStr for Point {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                .split(',')
                .map(|x| x.trim())
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point { x: x_fromstr, y: y_fromstr })
        }
    }

    let p: Result<Point, _> = "(3, 4)".parse();
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4 });

    println!("Success!");
}
