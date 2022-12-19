use std::io;
// C to F: F = C*(9/5) + 32
// F to C: C = (F-32)*(5/9)
fn main () {
    loop{
    println!("Do you want to convert to Celsius or Farenheit? Input C or F");

    let mut temp_input = String::new();

    // io::std readline
    // expect handling
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read input");
    
    let temp_input = match temp_input.trim() {
        "C" => "Celsius",
        "F" => "Farenheit",
        _ => continue,
    };

    // pass user variable into new variable

    println!("You want to convert to {temp_input}");
    println!("What temperature would you like to convert?");
    // variable for temp

    // io std read line
    // expect

    // convert temp into int include error hanlding

    // match loop
}
}

// celsius to farenheit fn

// farenheit to celsius fn