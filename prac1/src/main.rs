use std::fs::File;
//use std::io::Write;
use std::io::{self, Read, Write};

struct Car {
    model: String,
    year: u32,
    make: String,
    color: String,

}

//function to write to file 
fn create_and_write(Car: &Car)
{
    let mut file =File::create("user_info.txt").unwrap();
    writeln!(file,"The Model :{} ", Car.model).unwrap();
    writeln!(file,"The Year :{} ", Car.year).unwrap(); 
    writeln!(file,"The Make :{} ", Car.make).unwrap(); 
    writeln!(file,"The color :{} ", Car.color).unwrap(); 

}

// since its a very small file size, we will read entire file 
fn read_file()
{
    let mut file=File::open("user_info.txt").unwrap();
    let mut content = String:: new();
    file.read_to_string(&mut content).unwrap();
    println!("Car Content:\n{}", content);
}

fn reading_from_console() {
    let mut buffer = String::new();
//taking information from the user 
    print!("What is the model ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What is the year of the car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    print!("What is the make? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make= buffer.trim().to_string();
    buffer.clear();

    print!("What is the color? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

   let car = Car{model,year,make,color}; 
   // write to file 
   create_and_write(&car);
   // reading from file and printing it out to console 
   read_file();


}

fn main()
{
    reading_from_console();
}


