fn task1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s);

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

fn task2() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s;
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}

fn task3() {
    let s: String = String::from("hello, world!");
    let slice: &str = &s;
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn task4() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}


//vector

fn task5() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.push(3);

    assert_eq!(v1, v2);

    println!("Success!");
}

fn task6() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..3];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3];
    slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

fn task7() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn task8() {
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}

trait IpAddrTrait {
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn task9() {
    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

//hashmap

/*use std::collections::HashMap;

fn task10() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69.0);
    scores.insert("Katie", "58");

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}

use std::collections::HashMap;

fn task11() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2: HashMap<_, _> = teams.iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}

use std::collections::HashMap;

fn task12() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    m2.insert(v2.clone(), v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}
*/