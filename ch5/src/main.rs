fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };

    let smaller = Rectangle {
        width: 1,
        height: 7,
    };

    let larger = Rectangle {
        width: 40,
        height: 80,
    };
    println!("The area of the rectangle {:?} is {}", r, r.area());
    print!(
        "rectangle {:?} contains \n {:?}=> {}, \n {:?}=> {}, \n {:?}=> {}",
        r,
        smaller,
        r.contains(&smaller),
        larger,
        r.contains(&larger),
        r,
        r.contains(&r)
    )
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn contains(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}
