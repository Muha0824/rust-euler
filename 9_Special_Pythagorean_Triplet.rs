fn main() {
    let target = 1000;
    let val = find_product(target);
    println!("{val}");
}

fn find_product(target: i32) -> i32 {
    for a in 1..target {
        for b in (a + 1)..target {
            // Guarantee a + b + c = 1000
            let c = target - a - b;

            // Only check when c is larger than b
            if b < c {
                // Check Pythagorean property: a^2 + b^2 == c^2
                if (a * a) + (b * b) == (c * c) {
                    println!("Found triplet: a={a}, b={b}, c={c}");
                    return a * b * c;
                }
            }
        }
    }

    0
}