# migrating-from-c-to-rust-random-num-gen
Practice repo,  inspired by the tutorial series of Gary Explains on Youtube, teaching how to write RUST code which can be integrated in C code
[Link to the original tutorial by Gary Explains](https://www.youtube.com/watch?v=WsnFZk5-xwQ)

# How to compile and run
There are three directories within the repo.
1. c-code `Dedicated full C based implementation`. How to build and run.

    1. gcc -c vrandom.c 
    2. ar rcs libvrandom.a vrandom.o
    3. gcc -o main main.c L. -lvrandom

2. rust-code `This is the same program implemented in RUST`, just do cargo run.

3. c-rust-combo `This has the library code written in RUST, which is utilized in C directly, without modifying any parts of C code`, in order to build and run this one, we first have to compile and build the RUST library and then compile the C code using the RUST library.
    1. cargo build --release
    2. gcc -o main main.c -Ltarget/release -lc_rust_combo


Note: A point to be noted here is that this is still c-rust-combo still includes a unsafe rust version of code, and not is line with the true sprit of idiomatic RUST.

The `Gary Explains` tutorial also includes an example with idiomatic Rust, but for that to work we would also need to make changed to C code.
