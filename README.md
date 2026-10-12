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

---

## Running the Code

Run any specific problem binary with Cargo:

```bash
cargo run --bin 4_sum_square.rs
