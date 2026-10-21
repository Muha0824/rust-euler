fn main(){
    let nth_prime:usize = 10_001; 
    let val = is_prime(nth_prime); 
    println!("{val}"); 
}

fn is_prime(nth_prime:usize)-> i32{
    let mut counter:i32 = 2; 
    let mut prime_arr = vec![]; 

    while prime_arr.len() < nth_prime{
        let mut is_prime_check = true;

        for i in 2..=counter.isqrt(){
            if counter % i == 0{
                is_prime_check = false; 
                break;
            }
        } 

        if is_prime_check{
            prime_arr.push(counter); 
        }

        counter+=1; 

    }

    return prime_arr[prime_arr.len()-1];
}
