

fn main() {
    let end_nb = 1000; 
    let multi = [3,5]; 
    let mut arr_sum = vec![]; 

    for m in multi{
        let n = finding_n(m, end_nb);
        let multiples = sum(n,m); 
        
        for item in multiples{
            arr_sum.push(item);
        }
    }


    arr_sum.sort(); 
    arr_sum.dedup();
    let print_sum:i32 = arr_sum.iter().sum(); 
    

    println!("The total is {}", print_sum);


}


fn finding_n(num:i32,end_nb:i32)-> i32{
    if end_nb % num == 0{ 
        //println!("even"); 
        (end_nb/num)-1
    }
    else{
        //println!("odd"); 
        end_nb/num
    }
}

fn sum(n:i32, m:i32)->Vec<i32>{
    let mut list = vec![]; 
    for i in 1..=n{
        list.push(m * i);
    }
    list
}

