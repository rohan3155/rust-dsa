use rust_dsa::{Stack, stack};

fn main() {
    // --- Construction ---
    let mut s: Stack<i32> = stack![];
    let mut s2 = stack![1, 2, 3]; // 3 is on top
    let mut s3 = Stack::with_capacity(10);

    // --- Push ---
    s.push(1);
    s.push(2);
    s.push(3);

    // --- Pop (safe) ---
    if let Some(val) = s.pop() {
        println!("popped: {val}"); // 3
    }

    // --- Pop (unchecked) ---
    let val = s.pop_unchecked();
    println!("unchecked pop: {val}"); // 2

    // --- Peek (safe) ---
    if let Some(top) = s.top() {
        println!("top: {top}"); // 1
    }

    // --- Peek (unchecked) ---
    println!("top: {}", s.top_unchecked()); // 1

    // --- Mutate top in place ---
    s3.push(10);
    *s3.top_mut_unchecked() += 5;
    println!("mutated top: {}", s3.top_unchecked()); // 15

    // safe version
    if let Some(top) = s3.top_mut() {
        *top *= 2;
    }
    println!("doubled top: {}", s3.top_unchecked()); // 30

    // --- Inspect ---
    println!("empty: {}", s.is_empty());
    println!("len:   {}", s.len());
    println!("cap:   {}", s3.capacity());

    // --- Iterate (borrow, top → bottom, stack lives on) ---
    for val in &s2 {
        print!("{val} "); // 3 2 1
    }
    println!();

    // --- Iterate via .iter() explicitly ---
    let sum: i32 = s2.iter().sum();
    println!("sum: {sum}"); // 6

    // --- Iterate (owned, consumes stack) ---
    for val in s2 {
        // s2 moved here
        print!("{val} "); // 3 2 1
    }
    println!();
    // s2 no longer accessible here

    // --- Swap ---
    let mut a = stack![1, 2];
    let mut b = stack![9, 8];
    a.swap(&mut b);
    println!("a top: {}", a.top_unchecked()); // 8
    println!("b top: {}", b.top_unchecked()); // 2

    // --- Clear ---
    a.clear();
    println!("after clear: {}", a.is_empty()); // true

    // --- Debug ---
    let d = stack![1, 2, 3];
    println!("{:?}", d); // [3, 2, 1]  (top → bottom)

    // --- Trailing comma (fine) ---
    let _s = stack!["hello", "world"];
}
