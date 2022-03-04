#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //absolute crate path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

//reexportin hosting allows take reservation to use hosting
//rather than using sibling or absolute paths
pub use crate::front_of_house::hosting;

pub fn take_reservation() {
    hosting::add_to_waitlist();
}


fn serve_order() {}
mod back_of_house{

    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches") ,
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_breakfast() {
    //breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // change to wheat
    meal.toast = String::from("wheat");

    println!("yo quiero pan de {} por favor", meal.toast);

    //this won't compile b/c seasonal_fruit is private
    //meal.seasonal_fruit = String::from("berries");
}

pub fn order_apps() {
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}