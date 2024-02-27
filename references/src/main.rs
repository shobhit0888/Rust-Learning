fn main() {
    println!("Hello, world!");
    let mut x = 90;
    println!("The value of x is: {}", x);
    let  y = &mut x;
    *y += 1;
    // let z = &y;

    println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);
}
