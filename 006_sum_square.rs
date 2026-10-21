fn main(){
    let number_limit:i32= 100; 
    let diff = squares_sum(number_limit) - sum_squares(number_limit); 

    println!("The difference is {}", diff); 

}

fn sum_squares(number_limit:i32 ) -> i32 {       // 1^2 + 2^2 = 5    Sum of squares 
    let mut sum:i32 = 0; 
    let mut squared; 
    for i in 1..=number_limit{
        squared = i*i; 
        sum = squared + sum; 
    } 
    return sum; 
}

fn squares_sum(number_limit:i32) -> i32 {       // (1+2)^2 = 9      Squares of sum 
    let mut total:i32 = 0; 
    for i in 1..=number_limit{
        total = i + total; 
    }
    return total*total; 
}
