fn main(){
    let nb_below:i64 = 2_000_000; 
    let val = find_sum(nb_below);
    println!("{val}");  

}

fn find_sum(nb_below:i64)->i64{
    let mut counter:i64 = 2; 
    let mut prime_arr:Vec<i64> = vec![]; 
    

    while counter < nb_below{
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

    let sum:i64 = prime_arr.iter().sum(); 

    return sum; 

}