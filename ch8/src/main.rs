mod vec;

fn main() {
    println!("Hello, world!");
    let a = vec::new();
    println!("a {:?}", a);

    let v = vec![1,2,3,4,5];
    let third = &v[2];

    println!("3rd elem {}", third);
    match v.get(2) {
        Some(third) => println!("match 3rd"),
        None => println!("no match")
    }

    //will panic
    //let no_exist = &v[7];
    let dynamic_no_exit = v.get(7);
    match dynamic_no_exit {
        Some(&int32) => println!("it's not  none"),
        None => println!("element noexist")
    }
}
