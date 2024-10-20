fn task1() {
    let i = 3;
    {
        let borrow1 = &i;
        //
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn task2() {
    let result = longest("hello", "world");
    println!("The longest string is: {}", result);
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    // The lifetime 'a would last longer than `_x`, causing an error.
    // Remove the explicit lifetime annotation to make it work.
    let y: &'_ i32 = &_x;
}

fn task3() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn task4() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn task5() {
    let var_a = 35;
    let example: Example;

    {
        let var_b = NoCopyType {};
        example = Example { a: &var_a, b: &var_b };
        println!("(Success!) {:?}", example);
    }
}
#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn fix_me<'a, 'b>(foo: &'a Example<'a, 'b>) -> &'b NoCopyType {
    foo.b
}

fn task6() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!");
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn task7() {
    let excerpt = ImportantExcerpt { part: "Hello" };
    println!("Level: {}", excerpt.level());
}

/* Fill in the blank in two ways */
fn task1() {
    let v: &'static str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

fn task2() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    println!("static_string reference remains alive: {}", static_string);
}

/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn task3() {
    let i = 5;
    print_it(i);

    print_it(&i);

    print_it1(&i);

    print_it2(&i);
}

/* Annotate struct with lifetime: 1. `r` and `s` must have different lifetimes 2. lifetime of `s` is bigger than that of 'r' */
struct DoubleRef<'r, 's, T> {
    r: &'r T,
    s: &'s T,
}

fn task4() {
    println!("Success!")
}

/* Adding trait bounds to make it work */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn task5() {
    println!("Success!")
}

/* Adding trait bounds to make it work */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) {
    y = x;
    let _r: &'b &'a i32 = &&0;
}

fn task6() {
    println!("Success!")
}

/* Make it work by reordering some code */
fn task7() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref1 += 1;
    *ref2 += 2;

    println!("{}", data);
}

