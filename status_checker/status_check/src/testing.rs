#[cfg(test)]
// simple testing 
mod tests {
    use super::*; // import parent module 
    use std::time::Duration;
    use crate::monitor::check_website;
    #[test]
    // testing website access with google
    fn test_check_website_success() {
        let url = "https://google.com"; // url
        let timeout = Duration::from_secs(5); // timeout
        let result = check_website(url, timeout);// the result 
// checking if result.status is Ok aka successful
        assert!(result.status.is_ok());
    }
    
}
