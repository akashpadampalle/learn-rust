# intermediate 

## keywords
- struct
- () : tuples 


### create a structure or combine primitive types to make user defined type
```rust
sturct sturctName {
    data1: type,
    data2: type,
    data3: type,
    ...
}
```

### tuples 
```rust
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

// tuples are inclosed with ()

let numbers = one_two_three();
let (x, _y, _z) = one_two_three();

println!("{:?}, {:?}", x, numsbers.2); // x -> 1 and numbers.2 -> 3
```
we can use _ before unused identifier names (variable name) to calmdown rust compiler