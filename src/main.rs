use std::io;
use std::num::ParseFloatError;

fn main() -> Result<(), ParseFloatError> {
    println!("Welcome to the Rust Temperature Converter");
    println!("Please provide a number and unit you'd like to convert.");
    println!("Your input should be formatted as so: 100 F");
    // what the heck is the difference between String and str
    // why do I need to create a String to store the data?
    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");

    let v: Vec<&str> = input_number.split(' ').collect();
    // what is this {:?} stuff for?
    // println!("here is the vector: {:?}", v);

    // separate the first index and check if it's a number
    // turbo fish ::<>???
    // found type Result?!?!
    // why do I need match here?
    // do I do this each time I parse?
    // How do I terminate print to the console and give a better error message than ParseFloatError?
    let mut degrees: f32 = match v[0].trim().parse() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    // check that the second index is only a single letter, F or C
    // wow....
    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    // tried to add a feature .to_uppercase() the value the user put, but man...
    let unit = v[1].trim().chars().next().unwrap();

    // let mut unit = v[1].trim().chars();
    // match unit.next() {
    //     None => String::new(),
    //     Some(f) => f.to_uppercase().chain(unit).collect(),
    // }

    if unit == 'F' {
        // do the calc and return
        degrees = convert_to_celsius(degrees);
        println!("Your converted tempture is {:.2} C", degrees);
    } else if unit == 'C' {
        // do the calc and return
        degrees = convert_to_fahrenheit(degrees);
        println!("Your converted tempture is {:.2} F", degrees);
    } else {
        println!("You did not provide an appropriate unit.");
        println!("Please try again and provide either and F or C for your conversion");
    }

    // makes Ok() it look like a global?!?
    // how do I handle bubbling up the error if this wasn't in main?
    Ok(())
}

fn convert_to_fahrenheit(degrees: f32) -> f32 {
    // multiplying and dividing numbers of different types?!?!
    // oy vey
    (degrees * (9. / 5.) + 32.)
}

fn convert_to_celsius(degrees: f32) -> f32 {
    (degrees - 32.) * (5. / 9.)
}
