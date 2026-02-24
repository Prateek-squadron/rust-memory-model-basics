fn main() {
    // ----------------------------
    // 1️⃣ Immutable variable (Copy type)
    // ----------------------------
    let x = 10; // stack value
    let y = x;  // Copy occurs (i32 implements Copy)
    println!("x = {}, y = {}", x, y);

    // ----------------------------
    // 2️⃣ Mutable variable
    // ----------------------------
    let mut count = 5;
    count += 1;
    println!("count = {}", count);

    // ----------------------------
    // 3️⃣ Ownership (Heap data)
    // ----------------------------
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moved to s2

    // println!("{}", s1); ❌ would fail (moved)
    println!("s2 = {}", s2);

    // ----------------------------
    // 4️⃣ Borrowing (Immutable reference)
    // ----------------------------
    let s3 = String::from("World");
    print_length(&s3); // borrow, ownership NOT moved
    println!("s3 still valid: {}", s3);

    // ----------------------------
    // 5️⃣ Mutable Borrowing
    // ----------------------------
    let mut s4 = String::from("Rust");
    modify_string(&mut s4);
    println!("Modified s4 = {}", s4);

    // ----------------------------
    // 6️⃣ Ownership Transfer to Function
    // ----------------------------
    let s5 = String::from("Ownership");
    takes_ownership(s5);
    // println!("{}", s5); ❌ invalid, moved into function

    // ----------------------------
    // 7️⃣ Returning Ownership
    // ----------------------------
    let s6 = String::from("Return");
    let s6 = gives_back(s6);
    println!("Returned s6 = {}", s6);
}

// Function that borrows immutably
fn print_length(s: &String) {
    println!("Length = {}", s.len());
}

// Function that borrows mutably
fn modify_string(s: &mut String) {
    s.push_str(" Programming");
}

// Function that takes ownership
fn takes_ownership(s: String) {
    println!("Taking ownership of {}", s);
} // s dropped here

// Function that takes and returns ownership
fn gives_back(s: String) -> String {
    println!("Got {}", s);
    s
}