// fahrenheit 
fn fahrenheit_to_celsius(f: f64) -> f64{
 (f-32.0)*5.0/9.0 // expression 
}

fn celsius_to_fahrenheit(c: f64) -> f64
{
    (9.5/5.0)*c+32.0
}

fn main() {
    // declaring a const for freezing point of water 
    const FP : i32 = 32;
// mut variable 
let mut fTmp :f64 = 93.0;
//converting it to celsius and printing it out 
println!("Fahrenheit to Celsius: {}",fahrenheit_to_celsius(fTmp));

// loop 
let mut counter = 0;

loop{
    fTmp += 1.0; 
   println!("The conversion for: {}f is: {}c", fTmp, fahrenheit_to_celsius(fTmp));
   counter +=1;
   if counter == 5{
    break;
   }
}
 
}
