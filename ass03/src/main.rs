//function 
fn check_guess(guess:i32, secret:i32) -> i32
{
    if guess==secret {
       0
    }
    else if guess > secret {
        1
    }
    else{
        -1
    }
}

fn main() {
    // guess 
    let  secret = 78;
let mut num_guesses =0;

let mut guess = 76; 
        loop {
        
        num_guesses += 1;

        if check_guess(guess,secret) == 0
        {
             println!("Your guess is correct!");
                break;
        }
        else if check_guess(guess,secret) == 1{
            println!("Your guess is too high.")
        }
        else{
            println!("your guess is too high ")
        }
    // simulate user input 
    guess+=1;
    }

    println!("It took you {} guesses", num_guesses);
}

