# Rust

## Keywords

- let
- mut
- fn
- loop
- macro_rule!
- while
- if, else if, else
- break
- match
- enum

### variable and their types
```rust
/* vairalble types
    1. integer: [i32, i64] - whole numbers
    2. double or float
    3. boolean
    4. character
    5. string
*/
let var_name = value; // immutable variable
let mut var_name = value; // mutable variable
```

### println macro
```rust
// print any variable
println!("{:?}", var_name);

// {:?} - is placeholder for variable in string

// we can use {} to print string
println!("{}", str_1);
```

### how to create function
```rust
fn fun_name(arg1: data_type, arg2: data_type) -> return_type {
    // auto return last expression
    return value;
    // if we want to implictly return some value then don't add semicolon at last statement
    value
}
```

### how to create macro
```rust
macro_rule! macro_name {
    () => {
        println!("Hello, Macro!");
    }
}
```

### if else and match statement
```rust
if condition {} 
else if condition {} 
else {}

//exmaple
if age < 18 {
    println!("not aloud to drink");
}

// match is similer to switch
match var {
    v if v > 5 => println!(">5"),
    v if v < 5 => println!("<5"),
    v if v == 5 => println!("=5"),
    _ => println("not a valid statement"),
}
```
### loops in rust
```rust
// infinite loop 
loop {
    if condition {
        break;
    }
    // whatever work to do
}

// conditional loop
while condtion {
    // work to do
}
```

### enumatation or enum
```rust
// enum declaration
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// use of enum
Direction::Up
```

