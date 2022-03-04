fn main() {
    let home =IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Hello, world {:?} {:?}!", home, loopback);
    let m = Message::Write(String::from("hi there"));
    m.call();
    opts();
    let p = Coin::Penny;
    println!("penny value {}", p.value());
    assert_eq!(p.value(), 1);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    assert_eq!(six.unwrap(),6);
    assert_eq!(none.is_none(), true);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

fn opts() {
    let some_number:Option<i32> = Some(5);
    let some_string = Some("this string");
    let mut absent_num : Option<i32> =None;
    let x = some_number.unwrap_or(0)+absent_num.unwrap_or(0);
    absent_num = Option::from(x);
    println!("{:?}, {:?}, {:?}, {:?}", some_number, some_string, x, absent_num)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("pennies are small");
                1},
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,

        }
    }
}

fn plus_one(x:Option<i32>) ->Option<i32> {
    match x {
        None => None,
        Some(u) => Some(u + 1),
    }
}