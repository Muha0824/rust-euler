

fn main() {
    let end_nb = 795000; 
    let mut odd_nb = vec![]; 
    let mut sum = vec![]; 

    for i in 1..=end_nb{
        if i % 2 ==0{
            //pass
            }
            else{
                odd_nb.push(i)
            }
    }

    // println!("{:?}", odd_nb);

    for item in odd_nb{
        sum.push(item*item);
    }

     // println!("{:?}", sum); 
     
     let print_sum:i64 = sum.iter().sum(); 
     println!("The total is {}", print_sum);

}

