#[derive(Debug)]
enum GenderCat {
    Male, Female, Other
}
#[derive(Debug)]
struct Person{
    name: String,
    email: String,
    age: i32,
    gender: GenderCat,
}
fn main() {
    println!("Hello, world!");
    let person1 = Person{
        name: String::from("Shobhit Gupta"),
        email: String::from("shogupt2002@gmail.com"),
        age: 19,
        gender: GenderCat::Male,
    };
    println!("The value of person1 is: {:?}", person1);
}


// option enum
fn calculate_age(age: i32) -> Option<i32>{
    if age > 0 {
        Some(age)
    } else {
        None
    }
} 



