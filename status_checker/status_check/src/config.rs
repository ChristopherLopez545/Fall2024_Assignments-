use std::time::{Duration, SystemTime};

// structs to hold website results and config for the monitor process
#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,                  // URL of the website
    pub status: Result<u16, String>, // containinf the status code : either code (200) or error message
    pub response_time: Duration,     // time for response 
    pub timestamp: SystemTime,       // time stamp
}

// config parameters, makes it modular 
#[derive(Debug)]
pub struct Config {
    pub timeouts: Duration,  // timeout duration 
   // pub retries: usize,     // number of retires 
    pub num_threads: usize, // number fir worker threads
}

// default constructor 
impl Default for Config {
    fn default() -> Self {
        Self {
            timeouts: Duration::from_secs(5), // 5 sec timeout 
         //   retries: 3, // 3 max retries 
            num_threads: 4, // 4 worker threads 
        }
    }
}
