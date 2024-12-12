mod config;
mod worker;
mod monitor;
mod testing;
use config::Config;
use monitor::monitor_websites;
use std::time::Duration;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

// returns a vector filled with urls 
fn load_into_vec() ->Vec <String>
{
    let mut result = Vec::new();
    let mut file = File::open("site.txt").unwrap();
    let mut content = BufReader::new(file);

    for line in content.lines()
    {
        result.push(line.expect("Failed to read line!")); // will panic if the value is Result::Err
    }
    result
}

fn main() {
    // delcaring the cconfig 
    let config = Config {
        timeouts: Duration::from_secs(5), // timeout for each request being sent
        //retries: 2,                      // Number of retries whenever it failes 
        num_threads: 4,                  // number of workers 
    };

    // declaring the list of urls to monitor 
    let urls:Vec<String> = load_into_vec();

    // 
    println!("Monitoring has started !!...");
    let results = monitor_websites(urls, config);

    // displaying the capured results
    for result in results {
        match result.status {
            // status 200
            Ok(status) => println!(
                "Current URL: {}, Status: {}, Response Time: {:?}, Checked At: {:?}",
                result.url, status, result.response_time, result.timestamp
            ),
            // status has error 
            Err(err) => println!(
                "Current URL: {}, Error: {}, Checked At: {:?}",
                result.url, err, result.timestamp
            ),
        }
    }
}
