fn main() {
    println!("Hello, world!");
    let b = simple_box();
    println!("b={}", b);
}

fn simple_box() -> Box<i32> {
    Box::new(5)
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::simple_box;
    use crate::List::{Cons, Nil};

    #[test]
    fn simple() {
        let b = simple_box();
        assert_eq!(*b, 5);
    }

    #[test]
    fn list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn ref_practice() {
        let x = 5;
        let y = &x;
        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }
}
