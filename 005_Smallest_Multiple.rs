use std::time::Instant;

fn main(){
    let now = Instant::now();
    let mut final_nb:i64 = 20;
    let end_nb = 20; 

    'outer:loop{
        let mut count:i64 = 0; 
        for i in 2..=end_nb{
            if final_nb % i == 0{ 
                count += final_nb % i; 
            }
            else{
                count+=1;       // This prevents overflow of useless computation.
                break;
            }
        }

        if count == 0 {
            println!("The smallest number is {}", final_nb);
            break 'outer; 
        }

        final_nb += 20; 
    }

  let elapsed_time = now.elapsed();
  println!("The program took {}", elapsed_time.as_millis());

}


