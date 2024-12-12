use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::config::WebsiteStatus;
use crate::monitor::check_website;

// creating the thread pool 
// each thread HAS to 
/*
--Receive a url 
--Attempt to check website 
--send result back to main thread 
*/
pub fn create_thread_pool(
    num_threads: usize,
    job_receiver: Arc<Mutex<mpsc::Receiver<String>>>, // shared receiver 
    result_sender: mpsc::Sender<WebsiteStatus>,       // a sender to senf results 
    timeout: Duration,                                // timeout var
) {
    // creating the numbr of threads in this case its 4 
    for _ in 0..num_threads {
        // cloning the receiver and sender to have independent owner ship
        let job_receiver = Arc::clone(&job_receiver); 
        let result_sender = result_sender.clone();   

        // spawning a new thread with a closure 
        thread::spawn(move || {
            // this will stop when there is no more urls to process
            loop {
                // match receiver to a url
                let url = match job_receiver.lock().unwrap().recv() {
                    Ok(url) => url,
                    Err(_) => break, // BREAK if no urls 
                };

                // checking the website status 
                let status = check_website(&url, timeout);

                // sending the result back to the main thread 
                if result_sender.send(status).is_err() {
                    break;
                }
            }
        });
    }
}
