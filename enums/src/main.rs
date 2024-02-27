enum CarTypes{
    Hatchback,
    Sedan,
    SUV,
    Coupe,
}
fn car_type(car: CarTypes){
    match car {
        CarTypes::Hatchback => println!("Hatchback"),
        CarTypes::Sedan => println!("Sedan"),
        CarTypes::SUV => println!("SUV"),
        CarTypes::Coupe => println!("Coupe"),
    }
}
fn main() {
    println!("Hello, world!");
     car_type(CarTypes::Hatchback);
        car_type(CarTypes::Sedan);
        car_type(CarTypes::SUV);
        car_type(CarTypes::Coupe);
}
