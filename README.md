# 🦀 Rust Ownership and Borrowing

A focused Rust fundamentals project exploring the core memory safety concepts that make Rust unique — including ownership, move semantics, borrowing, references, and mutability.

This repository serves as a structured, hands-on demonstration of Rust’s memory model through small, controlled examples.

---

## 📚 Concepts Covered

- Variable bindings with `let` and `let mut`
- Stack vs Heap memory behavior
- Ownership rules
- Move semantics
- Immutable borrowing (`&T`)
- Mutable borrowing (`&mut T`)
- Function ownership transfer
- Returning ownership
- Scope-based automatic resource cleanup (`Drop`)

---

## 🧠 Why This Matters

Rust does **not** use a garbage collector.

Instead, it guarantees memory safety through:

- Single ownership
- Borrowing rules
- Compile-time enforcement

Understanding these fundamentals is essential for:

- Systems programming  
- Embedded development  
- High-performance backends  
- Security-focused software  
- Concurrent programming  

---

## 📂 Project Structure

```text
src/
└── main.rs
```

All examples are implemented inside `main.rs` for clarity and focused experimentation.

---

## 🚀 Running the Project

Ensure Rust is installed:

```bash
rustc --version
```

Run the project:

```bash
cargo run
```

---

## 🔍 Example Demonstrations

This project includes:

- Copy vs Move behavior (`i32` vs `String`)
- Ownership transfer into functions
- Borrowing without ownership loss
- Mutable borrowing with exclusive access
- Returning ownership from functions

Each section is intentionally isolated for clarity.

---

## 🏗 Core Ownership Rules

1. Each value has one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Violating these rules results in compile-time errors — preventing runtime memory bugs.

---

## 🎯 Learning Objective

The goal of this repository is not just to write Rust code, but to deeply internalize:

- How Rust prevents double-free errors  
- Why data races are prevented at compile time  
- How borrowing enables safe shared access  
- Why mutation requires explicit intent  

---

## 📈 Future Extensions

Planned expansions:

- Lifetimes  
- Smart pointers (`Box`, `Rc`, `Arc`)  
- Interior mutability (`RefCell`)  
- Concurrency primitives  
- Trait-based ownership patterns  

---

## 🛠 Built With

- Rust (stable toolchain)  
- Cargo  

---

## 📌 Author

**Prateek** — building strong foundations in systems programming and memory-safe software design.