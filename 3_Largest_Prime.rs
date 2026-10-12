fn main(){
    let mut a = 1; 
    let mut b = 2; 
    let limit = 4_000_000; 
    let mut arr = vec![a,b];
    let mut even_fibonaci = vec![]; 
    let mut next; 

    loop{
        next = a+b; 

        if next > limit{
            println!("That's enough"); 
            arr.len();  
            break 
        }

        arr.push(next); 

        a = b; 
        b = next; 

    }


    for item in arr{
        if item % 2 == 0{
            even_fibonaci.push(item);
        }
    }

    let print_sum:i64 = even_fibonaci.iter().sum(); 
    println!("The total is {}", print_sum);

}