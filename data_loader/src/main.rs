use ureq;
use serde::Deserialize;
//use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use reqwest;
use serde_json::Value;
use std::thread;
use std::time::Duration;


#[derive(Debug, Deserialize)]
struct Price 
{
    usd:f32,
}
impl Price
{
    fn new(price:f32)->Self
    {
        Self
        {
        usd:price,
        }
    }
}

// define the three structs 
#[derive(Debug, Deserialize)]
struct Bitcoin {
    bitcoin: Price,
}
#[derive(Debug, Deserialize)]
struct Ethereum{
    ethereum:Price,
}
#[derive(Debug, Deserialize)]
struct SP500
{
    sp500:Price,
}
impl Bitcoin
{
    fn new(price:Price)-> Self
    {
        Self
        {
            bitcoin: price,
        }
    }
}
impl Ethereum
{
    fn new(price:Price)-> Self
    {
        Self
        {
            ethereum: price,
        }
    }
}
impl SP500
{
    fn new(price:Price)-> Self
    {
        Self
        {
            sp500: price,
        }
    }
}
// next, create a pricing trait with fetch_price() and save to file method 
// fetch price is where we do the api call 
pub trait Pricing{
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price:f32);
    fn name(&self)-> &str; // this just returns the name o f the currency
}
// now implement it for each struct 
impl Pricing for Bitcoin
{
    // fetch and save will go here 
    fn fetch_price(&self) -> f32
    {
        // the url to grab the data 
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        //using ureq for call 
        let req = ureq::get(url).call().unwrap();
        let content = req.into_json::<Bitcoin>();
        // return the price 
        content.unwrap().bitcoin.usd
    }
    // save file: save the fetch price to a file 
    fn save_to_file(&self,price:f32)
    {
        // first check if the file is already created, if not create the file and add the first price
        let b = Path::new("bitcoin.txt").is_file();
        if !b
        {
                let mut file = File::create("bitcoin.txt").unwrap();
                writeln!(file, "{}",price).unwrap();
        } else{  // if already created, then just appened at the end of the file
            let mut file = OpenOptions::new()
              .append(true)
              .open("bitcoin.txt")
              .unwrap();
            // write to the file 
            writeln!(file,"{}",price).unwrap();
        }
       
    }
    fn name(&self) -> &str{
        "BIC"
    }
}

impl Pricing for Ethereum
{
    fn fetch_price(&self) -> f32
    {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        //using ureq for call 
        let req = ureq::get(url).call().unwrap();
        let content = req.into_json::<Ethereum>();
        // return the price 
        content.unwrap().ethereum.usd
    }
    // save file: save the fetch price to a file 
    fn save_to_file(&self,price:f32)
    {
        // first check if the file is already created, if not create the file and add the first price
        let b = Path::new("ethereum.txt").is_file();
        if !b
        {
                let mut file = File::create("ethereum.txt").unwrap();
                writeln!(file, "{}",price).unwrap();
        } else{  // if already created, then just appened at the end of the file
            let mut file = OpenOptions::new()
              .append(true)
              .open("ethereum.txt")
              .unwrap();
            // write to the file 
            writeln!(file,"{}",price).unwrap();
        }
       
    }
    fn name(&self) -> &str{
        "ETH"
    }
}

impl Pricing for SP500
{

     fn fetch_price(&self) -> f32
    {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        // fetch the json as a stirng with error handling with EXPECT 
        let response = reqwest::blocking::get(url) // using blocking from reqwest to make sync api calls 
            .expect("Failed to grab data"); // error handling this will print if the get fails 
            
     // !  IF LIMIT FOR REQUEST HITS ! if limit for api call is reached just return a hard coded value 
        if response.status().as_u16() == 429 //429 is the http code for "Too Many Request"
        {
           return 5948.71;
        }
        let response_text= response.text().expect("Failed"); // eorror handling for .text
   
        //println!("Response: {}", response_text); for debugging 

     // parse the string 
        let v: Value= serde_json::from_str(&response_text).expect("Failed to parse string");
        // checking if it can grab the nested price, if not print "failt to grab price"
        // chart -> result -> 0 -> meta -> regularMarketPrice 
        if let Some(price) = v["chart"]["result"][0]["meta"]["regularMarketPrice"].as_f64()
        {
         price as f32
        }
        else {
            println!("Failed to grab price for SP500");
            0.0 // return nothing 
        }
        
    }
    // save file: save the fetch price to a file 
    fn save_to_file(&self,price:f32){
       // first check if the file is already created, if not create the file and add the first price
       let b = Path::new("sp500.txt").is_file();
       if !b
       {
               let mut file = File::create("sp500.txt").unwrap();
               writeln!(file, "{}",price).unwrap();
       } else{  // if already created, then just appened at the end of the file
           let mut file = OpenOptions::new()
             .append(true)
             .open("sp500.txt")
             .unwrap();
           // write to the file 
           writeln!(file,"{}",price).unwrap();
       }
    }
    fn name(&self) -> &str{
        "SP500"
    }
}

fn main() {
    let mut price1= Price::new(0.0);// default values for now 
    let mut price2= Price::new(0.0);
    let mut price3= Price::new(0.0);
    // declaring the three currencies
    let mut bitcoin = Bitcoin::new(price1);
    let mut ethereum = Ethereum::new(price2);
    let mut sp500 = SP500::new(price3);
    // put these into a vector with dyn with trait 

    let three_curr: Vec<&dyn Pricing> = vec![&bitcoin,&ethereum,&sp500];

// Data Fetching Loop:
//     Iterate over each asset.
//     Fetch and save the latest pricing data.
//     Pause for 10 seconds.
// Dynamically dispatch the type of object
// loop
let mut i =0;

while i < 100
{
    for curr in three_curr.iter() {
        // fetch and save then pause 
        //print price to screen 
        let mut price = curr.fetch_price();
        println!("The price for {} is : {}",curr.name(),price);
        //save to file 
        curr.save_to_file(price);
        // pause 
        thread::sleep(Duration::from_secs(10));
    }
    i+=1;
}


}
