# Rust
## Background: 
Created in 2006 by software developer Graydon Hoare as a personal project while working at Mozilla Research. Its main emphases include performance, type safety, and concurrency. After its first stable release in May 2015, Rust gained popularity and was adopted by companies including Amazon, Discord, Dropbox, Google (Alphabet), Meta, and Microsoft.
## Common use cases: 
Rust has gained popularity across various domains due to its unique blend of performance, safety, and concurrency features. Commonly, Rust is employed in systems programming tasks, where low-level control and efficiency are important. Operating systems, web browsers, and game engines often leverage Rust for its ability to provide memory safety without sacrificing performance. Additionally, Rust is increasingly being adopted in the domain of backend web development, where its strong type system and concurrency capabilities enable the creation of highly scalable and reliable server-side applications. With its expressive syntax and tooling support, Rust is also finding utility in areas like embedded systems, where resource-constrained environments demand efficiency and reliability. 
## Influences: 
Rust drew inspiration from functional programming concepts like immutability, higher-order functions, and algebraic data types.
## Key features
#### Memory Safety: 
Rust enforces memory safety, meaning that all references point to valid memory without relying on a garbage collector. Rust gives you the choice of storing data on the stack or on the heap and determines at compile time when memory is no longer needed and can be cleaned up. This allows efficient usage of memory as well as more performant memory access.
#### Borrow Checker: 
To prevent data races and ensure memory safety, Rust’s “borrow checker” tracks object lifetimes during compilation. This is the part of the compiler responsible for ensuring that references do not outlive the data they refer to, and it helps eliminate entire classes of bugs caused by memory unsafety.
#### Linux Kernel Support: 
In December 2022, Rust became the first language other than C and assembly to be supported in the development of the Linux kernel.
## Pros
#### Memory safety and ownership model: 
Rust’s ownership system ensures memory safety by preventing common programming errors like null pointer dereferences and data races. The borrow checker enforces strict rules for mutable and immutable references, reducing runtime errors.
#### Performance: 
Rust compiles to efficient machine code, making it suitable for performance-critical applications. It offers predictable performance without sacrificing safety.
#### Embedded and Bare-Metal Development: 
Rust provides direct access to hardware and memory, making it ideal for low-level development. It shines in environments like operating system kernels and microcontroller applications. 
#### Predictable Behavior: 
Rust’s type system allows you to predict runtime behavior more accurately. Compile-time checks catch potential issues before execution, reducing debugging time.
Generics and Abstraction: Rust supports powerful generics, enabling code reuse and flexibility. Its expressive type system allows for concise and reusable abstractions.

## Cons
Steep learning curve, slow compilation time
#### Libraries: 
While Rust does provide a number of libraries (commonly referred to as “Crates” in the rust community) that can be used for things like web development, database interaction, and gaming, it is still fairly new. So, that being said, there are not nearly as many libraries out there for Rust as there are for languages like Javascript or Python.
