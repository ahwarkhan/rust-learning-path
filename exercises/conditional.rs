#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age {
    New,
    Used,
}

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Conditional expression to determine car age
    if miles > 0 {
        (Age::Used, miles) // Used car with current mileage
    } else {
        (Age::New, 0) // New car with zero miles
    }
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Show details about car order
    // Check if order is for Used or New car, then check the roof type
    if miles > 0 {
        // Used car
        if roof {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        } else {
            println!("Prepare a used car: {:?}, {}, Convertible, {} miles\n", motor, color, miles);
        }
    } else {
        // New car
        if roof {
            println!("Build a new car: {:?}, {}, Hard top, 0 miles\n", motor, color);
        } else {
            println!("Build a new car: {:?}, {}, Convertible, 0 miles\n", motor, color);
        }
    }

    // Create a new "Car" instance
    Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
