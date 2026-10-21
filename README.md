# Learning Rust with Project Euler 🦀

This repository tracks my journey learning Rust from scratch through Project Euler challenges. Rather than just watching tutorials, I'm learning by writing code, breaking things, and fixing logic bugs myself.

---

## Real Errors & Lessons Learned

### 1. Integer Limits (`i32` vs `i64`)
* **The Issue:** Large numbers caused "arithmetic overflow" panics.
* **The Lesson:** Rust defaults to 32-bit integers (`i32`). Exponential growth and large sums need 64-bit integers (`i64`).

### 2. Execution Order in Loops
* **The Issue:** Pushing values to vectors *before* checking limits caused off-by-one errors and extra items in arrays.
* **The Lesson:** Gatekeeper checks (`if next > limit { break; }`) must run **before** pushing values or mutating core state.

### 3. Keeping State Variables in Sync
* **The Issue:** Calculating remaining targets using old totals caused infinite loops during prime factorization.
* **The Lesson:** Always update dependency variables in exact sequential order so calculations never run on stale data.

### 4. Directing Logic to the Right Target
* **The Issue:** Checking divisibility against the original target (`end_nb`) instead of the shrinking remainder (`remaining`) let non-prime composite numbers slip through.
* **The Lesson:** Test loop conditions against the active, moving state variable rather than the static original input.

### 5. Writing Modular Helper Functions
* **The Issue:** Putting every loop and calculation inside `fn main()` made the code disorienting to review later.
* **The Lesson:** Extract logic into small, dedicated helper functions (e.g., `sum_squares(limit)`). It keeps `main()` clean and self-documenting.

### 6. Optimizing Search Space (Project Euler Problem 5)
* **The Problem:** Finding the smallest number evenly divisible by all numbers from 1 to 20.
* **The Benchmarks:**
  * **First Draft (`step = 1`):** Ran in **6.3 seconds** (on average over 10 tests) because it checked every single integer up to 232 million.
  * **Optimized Draft (`step = 20`):** Ran in **679.5 ms** (on average over 10 tests) by starting at 20 and stepping forward by 20, skipping the mathematically impossible candidates.
* **The Lesson:** Always follow the classic engineering rule: *Make it work, make it right, make it fast*. Shrinking my search space made the code run faster. 


### 7. Avoiding the "Snapshot" Trap (Project Euler Problem 7)
* **The Problem:** Finding the 10,001st prime number by collecting primes in a vector using a while loop.
* * **The Issue:** Calculating state variables (`like let check_prime_len = prime.len()`) before entering the loop created a static snapshot in memory (`0`). Because the variable was never updated inside the loop, the condition remained stuck at `0 < 10_001` forever, locking the program in an infinite loop.
  * **The Lesson**: Variables in Rust store fixed values in memory, not live dynamic formulas. Always evaluate loop conditions directly in the while header (`while prime.len() < 10_001`) or explicitly update dependency variables inside the loop body.
