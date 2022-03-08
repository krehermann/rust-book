use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
/// Following point to the value via Deference Operator
/// this is simple example of deferencing: create a ref to an i32
/// ```
///
/// let x = 5;
/// let y = &x;
/// assert_eq!(x, 5);
/// assert_eq!(*y,5);
/// ```
///
fn z() {}

#[derive(Debug)]
pub struct MyBox<T: Debug>(T);
impl<T: Debug> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
/// Creating our on box implementation
///
/// # Examples
///
/// Basic dereferencing.
/// Under the covers rust replaces *y -> *(y.deref())
/// With this in mind, it must be that the y.deref() itself a ref
/// Thinking a little, this must be true to be consistent
/// with the ownership model. If y.deref() were to be value,
/// that would require move'ing the value out of MyBox,
/// and we don't want to take ownership.
/// ```
/// use ch15::MyBox;
/// let x = 5;
/// let y = MyBox::new(x);
/// assert_eq!(x,5);
/// assert_eq!(*y,5);
/// ```
///
/// Deference Coercion
///
/// ```
/// use ch15::MyBox;
/// {
/// fn hello(name: &str)  {
/// println!("Yo, {}!", name);
/// }
/// let m = MyBox::new(String::from("Rustifarian"));
/// // Deref coercion &<MyBox<String>> -> &<MyBox<String>>.deref() -> &<String> -> (stdlib) &str
/// hello(&m);
/// //equivalent code with deref coercion
/// // *m => *<MyBox<String>> -> <String>; &(String)[...] -> &str equal to whole slice of String
/// hello(&(*m)[..])
/// }
/// ```

impl<T: Debug> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping my box with data `{:?}`", *self)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/// Rc<T> & reference counting
/// reference counting enables multiple ownership, as is needed in data structures
/// such as graphs
///

enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::SharedList::*;
    use crate::{CustomSmartPointer, MyBox};

    #[test]
    fn shared_list() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        assert_eq!(1,Rc::strong_count(&a));
        // use Rc::clone it increase the ref counter. It is possible to use a.clone, but
        // that is an anti pattern. Rc::clone makes clear the intention of increasing the
        // counter whereas a.clone often results in a deep copy.
        let b = Cons(3, Rc::clone(&a));
        assert_eq!(2,Rc::strong_count(&a));
        {
            // this next line doesn't work with a Box bc `a` has been moved.
            let c = Cons(4, Rc::clone(&a));
            assert_eq!(3,Rc::strong_count(&a));
        }
        assert_eq!(2,Rc::strong_count(&a));
    }
    #[test]
    fn print_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    #[test]
    fn print_drop_box() {
        let m = MyBox::new("hiddy ho");
        let n = MyBox::new(7);

        println!("Boxes created.");
        println!("force dropping {:?}", m);
        drop(m);
        println!("other box still around {:?} -> {}", n, *n)
    }
}
