use std::time::{Duration, Instant, SystemTime};
use std::sync::{mpsc, Arc, Mutex};
use crate::config::{Config, WebsiteStatus};
use crate::worker::create_thread_pool;
use ureq;

// grabs the status and data from the urls 
pub fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start_time = Instant::now(); // start the timer for response time 

    // sending a get request to the cur url 
    let response = ureq::get(url)
        .timeout(timeout) // wait 
        .call();

    // checking the status of the website 
    let status = if let Ok(response) = response {
        Ok(response.status()) // if successful it returns the http status code 
    } else {
        Err(response.unwrap_err().to_string()) // erroe handling if the request fails, convert to string 
    };

    // creating a struct to hold the http request information , from config.rs
WebsiteStatus { // return 
        url: url.to_string(), // the curr url
        status,
        response_time: start_time.elapsed(),
        timestamp: SystemTime::now(), //capturing the time stamp for the request 
    }
}
// creating a thread pool with a channel between them
pub fn monitor_websites(urls: Vec<String>, config: Config) -> Vec<WebsiteStatus> {
    // sender = sends urls, receiver = receives the urls 
    let (worker_sender, worker_receiver) = mpsc::channel();
    // channel for worker threads to send back the results they capture 
    let (result_sender, result_receiver) = mpsc::channel();
// Important! allowing safe shared acces to data across the threads
    let worker_receiver = Arc::new(Mutex::new(worker_receiver));

    // creating the threads 
    create_thread_pool(
        config.num_threads, // 4 but this can chang e
        worker_receiver.clone(),
        result_sender, 
        config.timeouts, // timeout for each website 
        //config.retries, // num of reties . 2 for rn
    );

    // sending the url to the worker threads 
    for url in &urls {
            // clone the curr url so each thread can have its own version 
        worker_sender.send(url.clone()).unwrap(); // 
    }

    // SHUTDOWN the pool
    drop(worker_sender);

    // collecting what the threads captured 
    let mut results = Vec::new();
    for _ in 0..urls.len() {

        if let Ok(result) = result_receiver.recv() {
            // push the results into the vector 
            results.push(result);
        }
    }
    // return 
    results
}
