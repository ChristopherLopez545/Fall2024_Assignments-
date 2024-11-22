# Data Loader

## Description

    - Take data from API and store it in a file

How fetch_price() works
used in class example for Bitcoin and Ethereum

Used ureq to grab(get request) the json and parse it.
into_json<Bitcoin> navigates to the bitcoin section with the help of structs
Ex
// the url to grab the data
let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
let req = ureq::get(url).call().unwrap();
let content = req.into_json::<Bitcoin>();
content.unwrap().bitcoin.usd

# Since the SP500 json structure is lengthy, I looked for a different approach to parse through the json structure.

Source: https://whoisryosuke.com/blog/2022/parsing-json-with-rust

    '''rust
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    '''

We can see that they're grabbing the specific field in the print statement in a simple matter
v["name"] -> John Doe
v["phones"][0] -> +44 1234567

# I incorporated that in my use case

'''rust
// parse the string
let v: Value= serde_json::from_str(&response_text).expect("Failed to pars");
// checking if it can grab the nested price, if not print "failt to grab price"
// chart -> result -> 0 -> meta -> regularMarketPrice
if let Some(price) = v["chart"]["result"][0]["meta"]["regularMarketPrice"].as_f64()
'''
Following the json structure price will be : price= chart -> result->0 -> meta->regularMarketPrice
