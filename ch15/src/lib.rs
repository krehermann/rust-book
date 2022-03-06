
use std::ops::Deref;
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

pub struct MyBox<T>(T);
impl<T> MyBox<T> {
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
/// fn hello(name: &str)  {
/// println!("Yo, {}!", name);
/// }
/// let m = MyBox::new(String::from("Rustifarian"));
/// // Deref coercion &<MyBox<String>> -> &<MyBox<String>>.deref() -> &<String> -> (stdlib) &str
/// hello(&m);
/// //equivalent code with deref coercion
/// // *m => *<MyBox<String>> -> <String>; &(String)[...] -> &str equal to whole slice of String
/// hello(&(*m)[..])
/// ```

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
