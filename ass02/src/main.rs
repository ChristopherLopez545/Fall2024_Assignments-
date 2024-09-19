fn is_even(n:&i32) -> bool
{
    if n % 2 == 0
    {
        return true;
    }
    else {
        return false;
    }
}


fn main() {

    let nums =  [23,90,21,15,101,56,32,20,1,44];
    //for loop to iterate 
    for num in nums.iter()
    {
        let even_odd = if is_even(&num)
        {
            "even"
        }
        else{
            "odd"
        };
        println!("The number {} is: {}",num, even_odd);
        if num % 3 ==0 && num % 5 == 0
        {
            println!("FizzBuzz");
        }
        else if num % 5 == 0
        {
            println!("Buzz");
        }
         else if num % 3 ==0
        {
            println!("Fizz");
        }
    }
    // finding the sum of all numbers in the array 
    let size = nums.len();
    let mut sum = 0 ;
    let mut counter = 0;
    while counter < size {
        sum+=nums[counter];
        counter +=1;
    }
    println!("The sum is:{} ",sum );

    // finding the largest number in th array 
    let mut i = 0;
    let mut current_num = nums[i];
    loop{
        if i == 10{
            break;
        }
        if nums[i] > current_num{
            current_num = nums[i];
        }
        
        i+=1;
    }
    println!("The largest number in the array is: {}", current_num);
}
