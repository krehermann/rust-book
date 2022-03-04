

fn main() {
    let s =String::from("hello world");
    println!("{}", s);
    let f =first_word(&s);
    println!("first word is '{}'", f)
}

//&str is a string slice
fn first_word(s: &str)->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}

/*
//dangle returns a reference to out of scope variable
//it is illegal and won't compile
fn dangle() -> &String {
    let s = String::from("heyo");
    &s
}
*/

fn take_ownership(s: String) {
    println!("'{}' is now mine. mawahahah", s);
}

fn ref_out_of_scope() {
    let s =String::from("my name is");
    take_ownership(s);
    println!("this won't compile b/c '{}' is gone ",s)
}

//primitive types are copied into the stack when
//passed as arguments, unlike non-primitives that
//use the heap and are drop'd upon passage
fn make_copy(i: i32) {
    println!("{}", i)
}

fn ref_a_copy() {
    let i :i32 = 1;
    make_copy(i);
    println!("{} is still in scope b/c it's primitive", i);
}