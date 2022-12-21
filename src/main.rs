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
        if temp_input.trim() == "exit" {
            break;
        }

    
    let temp_input = match temp_input.trim() {
        "C" => "Celsius",
        "F" => "Farenheit",
        _ => continue,
    };

    // pass user variable into new variable

    println!("You want to convert to {temp_input}");
    println!("What temperature would you like to convert?");
    let mut temp_num = String::new();

    io::stdin()
        .read_line(&mut temp_num)
        .expect("Failed to read line");
        if temp_num.trim() == "exit" {
            break;
        }
    
    let temp_num: f64 = match temp_num.trim().parse() {
        Ok(temp_num) => temp_num,
        Err(_) => {
            println!("Please enter a valid temperature");
            continue;
        }
    };

    match temp_input.trim() {
        "Celsius" => println!("{}", ftoc(temp_num)),
        "Farenheit" => print!("{}", ctof(temp_num)),
        _ => println!("Temp = {:?}", temp_input)
    }
  }
}

fn ctof(celsius: f64) -> f64 {
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    return fahrenheit;
}

fn ftoc(fahrenheit: f64) -> f64 {
    // To convert a temperature from Fahrenheit to Celsius,
    // subtract 32 and then multiply by 5/9
    (fahrenheit - 32.0) * 5.0 / 9.0
}