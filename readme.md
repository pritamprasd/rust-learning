# RUST notes & sample code
- `statically typed` : must know the types of all variables at compile time
- `impossible dangling pointer` : memory safety guarantees without needing a garbage collector.
- `default immutability`: 
- `default immutable references`: can't change a borrowed value.
- `Reference mutability`: At any given time, you can have either one mutable reference or any number of immutable references (preventing data race).
- Memory Allocation/Cleanup: Owner of data automatically clean up that data when the owner goes out of scope.
    - `C++`: Programmers Responsibility
    - `Rust`: Algorithm/compiler's responsibility.
    - `Java`: Machine's Responsibility(GC)

## Language specific rules
### Data types & others
- `&` : reference.
- `::` in `String::new()` indicates that new is an `associated function` of the String type. An associated function is a function that’s implemented on a type.
- `Constants`(vs `immutable_vars`): may be set only to a constant expression, can be declared in global scope.
- `Shadowing`(vs `mut`): we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
- Unlike languages such as Ruby and JavaScript, Rust `will not automatically try to convert` non-Boolean types to a Boolean
- `integer overflow`:
    - `debug`: panic at runtime
    - `release`: two’s complement wrapping    
#### Data Types
- `scaler`(single value): `integers` (i8 to i128 and u8 to u128), `floating-point numbers`(f32, f64), `Booleans`, `characters`(4byte).
- `compound`:
    - `tuples` (): rouping together a number of values with a variety of types into one compound type, fixed length.
    - `arrays` []: same type, fixed length.
        - `let a = [3; 5]` == `let a = [3, 3, 3, 3, 3]`
        - When we access an element using indexing(`idx`), Rust checks `if idx < arr_len`, If the `idx>=arr_len` Rust will panic(disallowing memmory access).
- `slice`(subarray): reference a contiguous sequence of elements in a collection rather than the whole collection.(No ownership). `[starting_index..ending_index]`.
- 

### Functions
- best practice: `snake_case`, 
- `Function` bodies are made up of a series of statements optionally ending in an expression.
    - `Statements` are instructions that perform some action and do not return a value (Function definitions are also statements). `let x = (let y = 6)` don't work in Rust as like C and Ruby.
    - `Expressions` evaluate to a resulting value. Calling a function, If, Calling a macro, A new scope block created with curly brackets is an expression.
    - `x + 1` is a expression but `x + 1;` is a statement.
- `return`: don’t name return values, but we must declare their type after an arrow `->`, return the last `expression` implicitly.

### Loops (loop, for , while)
- `loop` : infinite looping, return values like `break <val>`.
    - `loop labels`: If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

### Ownership
#### Stack vs Heap
- Pushing to the `stack`(LIFO) is faster than allocating on the `heap` because the allocator never has to search for a place to store new data
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

#### Rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.    
    
#### Explaination
- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable. Rust calls `drop` automatically at the closing curly bracket. (This can lead to double free error, so Rule 2).
    ```rust
    // MOVE
    {
        let s1 = String::from("hello");
        let s2 = s1;                    // move s1 to s2 (no shallow copy)
        println!("{}, world!", s1);     // Error
    }
    // CLONE
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();            // deep copy, BAD for performance
        println!("s1 = {}, s2 = {}", s1, s2);   // Valid
    }
    // COPY: stack-only, as known-size data types are stored in stack
    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y); // Valid
    }
    ```
- If a type implements the `Copy` trait, variables that use it `do not move, but rather are trivially copied`, making them still valid after assignment to another variable. 
    - Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
    - As a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    - ints, bool, floats, char, Tuples(if they only contain types that also implement Copy).
- Passing a variable to a function will move or copy, just as assignment does.
    ```rust
    fn main() {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s);             // s's value moves into the function, and so is no longer valid here

        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    ```

#### References(valid pointer) & Borrowing(creating a reference)
- What if we want to let a function `use a value` but `not take ownership`.
- A `reference` is like a pointer in that it’s an `address` we can follow to access the data stored at that address; that data is `owned by some other variable`. Unlike a pointer, `a reference is guaranteed to point to a valid value` of a particular type for the life of that reference.
- refers to a value but does not own it -->  Because it does not own it, the value it points to will not be dropped when the reference stops being used
- Just as variables are `immutable by default`, so are references. We’re not allowed to modify something we have a reference to.
- `mutable reference`: modify a borrowed value.  
    - if you have a mutable reference to a value, you can have no other references to that value. (`prevent data races` at compile time).
        ```rust
        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{}, {}", r1, r2); //Error: cannot borrow `s` as mutable more than once at a time
        ```
    - As always, we can use curly brackets to create a new scope, allowing for multiple mutable references.
        ```rust
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s;
        ```
    - We also cannot have a mutable reference while we have an immutable one to the same value.
        ```rust
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM
        ```
    - A reference’s scope starts from where it is introduced and continues through the `last time that reference is used`. Below code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced.
        ```rust
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);  // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
        ```
- 




### Macro


### Gotchas:
- In Rust, variables are immutable by default. (`mut` makes a var mutable).
- In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
- Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
- A `data race` happens when these three behaviors occur: 
    1. Two or more pointers access the same data at the same time.
    1. At least one of the pointers is being used to write to the data.
    1. There’s no mechanism being used to synchronize access to the data.
- `Dangling pointer`:
    ```rust
    // ### Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    fn dangle() -> &String { // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String
        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

    // ### Valid
    fn no_dangle() -> String {
        let s = String::from("hello");
        s   // Ownership is moved.
    }
    ```
- 


## Cargo:
```sh
cargo new <name>        # new project
cargo update            # updates crates
cargo doc --open        # dep docs offline
cargo check             # no executable
cargo build             # build debug exe
cargo build --release   # build optimized exe
cargo run               # run main.rs
```



## Misc
#### Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
#### Bookmark Reading
```sh
rustup docs --book
```
- Preludes(bydefault included): `/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch04-03-slices.html`
- 

####

