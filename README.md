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

# Cargo package manager for rust

cargo is used to build the project or to install dependencies

## create project 
```bash
cargo new package_name
```

## run project
```bash
cargo run --quiet
```
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

# Memory Concepts

## ownership
rust uses the ownership model to prevent memory leaks. In which function the variable is create, that function is the owner of that particular variable. Management and clearing that variable from memory is the duety of that function.
In rust we can **move** or **borrow** the variables ownership

```rust
enum Light { Bright, Dull }

fn print_light(Light: light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let light = Light::Dull;
    print_light(light);
    print_light(light);
}
```

in the above code snippet there will be the error light variable is declared inside the main function. that's why the `main` function is `owner` of `light`. `print_light(light)` in this statement we are `moving` the `ownershipt` to the `print_light` function. so print_light function will use that variable and at the end of the function print_light function will delete the variable from memory. That's how there will be `no reference remain` of light in in main function or in print_light function and sencond time when we use print_light function it will give us error cause light doesn't have any reference in memory.

```rust
rust
enum Light { Bright, Dull }

fn print_light(Light: &light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let light = Light::Dull;
    print_light(&light);
    print_light(&light);
}
```

This is how we can refactor code to get rid of error. in this case we are just `borrowing` light variable by using `&` symbol in front of variable name. In this scenario oweing (givine referece) to print_light function and owener of light is still main function. print_light can only use this variable but can't delete from memory.

## impl

implement new functionality to existing struct or enum

```rust
struct Temprature {
    deg: f64
}

impl Temprature {
    fn create(deg: f64) -> Self {
        Self { deg }
    }

    fn print(&self) {
        println!("{:?}", self.deg);
    }
}

fn main() {
    let temp1 = Temprature {deg: 34.0};
    let temp2 = Temprature::create(66.2);

    temp1.print();
    temp2.print();
}
```

In this above example we are adding new functionality `create` and `print` to our Temprature struct. if function is taking `&self` as argument then we can call this function using variable and self represents the current struct or enum if not we can use that function using struct name name and `::` syntax.

if create function we are creating new Temprature (represented by `Self`) and in print function we are printing the value of degree.


## Vector
vectors are same as array in other programming language. In rust vector are homogenious that means only same kind of data can be stored togather. we can store enums and struct also.

```rust
// creating vector using macro vec![]
let v1 = vec![1,2,3];

// we can also create using factory method
let mut v2 = Vec::new();
v2.push(1);
v2.push(2);
v2.push(3);
v2.pop();
v2.len();

// we can iterate over vectors in using for..in loop
for num in v2 {
    println!("{:?}", num);
}
```

In this above i have demonstrate two ways to create vectors 
1. vec! macro
2. Vec::new() factory function

we have additional function such as `len` `push` `pop` to work with vector
we can use `for...in` loop to iterate. for...in loop moves the variable's owenership. so, after fininshing for...in loop we no longer use that vector in our main function. Instead we can `borrow` variable using `&`.

## String Datatype

there are two types of strings in rust 
- &str: string slice
- String: own string

```rust
fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
// when we use "" to create string this will create a string slice &str.
let str1 = "my string";

//there are multiple ways to create a owned string.
let str2 = "some string value".to_owned();
let str3 = String::from("another string");

// by defauld string slice sends reference or borrow.
print(str1);

// we have to use & in front of owned string to borrow.
print(&str2);
}
```
If we are using String **inside struct we have to use owned string we cannot use string sllice.** while sending string to function we use string slice because it is more efficient.

## Derive macro

derive is macro to add atomatic functionality to enums and strucs for example `#[derive(Debug)]` by using this macro enum and struc can become printable using debuging string {:?}

```rust
#[derive(Debug)]
enum Color {
    Green,
    Red,
    Blue
} 

#[derive(Debug)]
struct Person {
    name: String,
    color: Color
}

fn main() {
    let p1 = Person { 
        name: String::from("akash"), 
        color: Color::Red 
    };
    println!("{:?}", p1);
}
```

we have other derive macros such as `Clone` and `Copy` by using this derives whenever we send a ownership to other function (move) it automatically create copy of that struct or enum and send it instead of original variable. *only use this derives with small structures otherwise we will unknowingly create expensive copies*.

```rust
#[derive(Debug, Clone, Copy)]
enum Color {
    Green,
    Red,
    Blue
} 

#[derive(Debug, Clone, Copy)]
struct Person {
    name: String,
    color: Color
}

// we are not taking any reference here.
fn print_person(data: Person) {
    println!("{:?}", data);
}

fn main() {
    let p1 = Person { 
        name: String::from("akash"), 
        color: Color::Red 
    };
    println!("{:?}", p1);
    // here copy of p1 will be create automatically ans sended to the print_person function.
    print_person(p1);
    // we will not get any error because we are sending copies to print_person function and origin p1 is owned by main function.
    print_person(p1);
}
```

## Type annotations

rust compiler automatically infer the type of variable but sometime we have to explicitly specify the type of variable for example when creating a function. syntax is very simple `var_name: var_type`.
generic types are inclosed inside angular brackets `<>`

```rust
fn print_str(data: &str) {}

fn main() {
    let nums: Vec<i32> = vec![];
}
```

# Advance

## Revisit enums 
enus represent varients. enums can have optionly hold data

```rust
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}
```
## Advance Match

### Matching on enum

```rust
fn main() {
    let n = 3;
    match n {
        3 => println!("{:?}", 3),
        other => println("other number: {:?}", other),
    }
}
```
In above match statement we have `variable` name instead of `_` in match arm. Underscore means ignoring something in rust. point of discusion is that we can get all possible values into a variable 😅.

```rust
enum Discount {
    Percent(i32),
    Flat(i32),
}

fn main() {
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2");
        Discount::Flat(amount) => println!("flat discount of: {:?}", amount);
        _ => (),
    }
}
```

`_ => (),` means ignoring everything else. here we use it to ignore all Percent varient of Discount.

### Matching on struct

```rust
struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let t1 = Ticket { 
        event: "diwali".to_owned(), 
        price: 20,
    };

    match t1 {
        Ticket { price: 50, event } => println!("event @ 50 {:?}", event),
        Ticket { price, .. } => println!("price = {:?", price),
    }

}
```

Here, `..` means ignoring other values in struct.

## Option - working with data

option is used to denote the data that is optional. it  is an enum with two fields Some(T) and None.

```rust
// option syntax
enum Option<T> {
    Some(T),
    None,
}
```
Option is enum so we have to use with Option::Some(data) or Option::None But options are so frequently used in rust so drop the Option declaration in front of varient and can directly use Sone(data) or None.

```rust
struct Customer {
    age: Option<i32>,
    email: String
}

fn main() {

    let customers: Vec<Customer> = vec![
        Customer {
            age: Some(40),
            email: "akash.padampalle@gmail.com".to_owned(),
        },
        Customer {
            age: None,
            email: "someone@hotmail.com".to_owned(),
        }
    ];

    for customer in customers {
        match customer {
            Customer { age: Some(value), email } => println!("email: {:?}, age: {:?}", email, value),
            Customer { age: None, email } => println!("email: {:?}", email), 
        }
    } 
}
```
## Documentation
`///` we can use thriple forword slashes to write documentations string.
After adding documentations string we can create documentations automatically using cargo command
`cargo doc --open`

## Standard liabraries

we can access the local documentation of rust using `rustup docs` command on bash terminal.

## Result - Working with data
### defination
```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
``` 
### Explaination
Result are used to handle the exceptions it will contains either Ok with some data or Err with error data.

example:
```rust
#[derive(Debug)]
enum SoundData {
    Alert
}

fn get_sound(sound: &str) -> Result<SoundData, String> {
    if sound == "alert" {
        return Ok(SoundData::Alert)
    } else {
        return Err("unable to find sound".to_owned())
    }
}

fn print_sound(sound: &SoundData) {
    println!("{:?}", sound);
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(inner_sound) => print_sound(inner_sound),
        Err(message) => println!("{:?}", message),
    }

    let sound2 = pick_sound("alert");
}
```

there is shorter way to handle Result using question mark operator `?`

```rust
fn pick_sound(input: &str) -> Result<(), String> {
    /* by using ? we check weather Result is okay or not.
    if result is Err then it will return Result<Err(message)>.
    or if result is okay then the value of Ok will put inside sound variable.
    */
    let sound = get_sound(input)?;
    // here we are assured that sound is type of SoundData.
    print_sound(&sound);

    // returning ok with nothing to satisfy compiler.
    Ok(())
}
```

how quetion mark is working behind the curtain.
```rust
// lets have look this line
let sound = get_sound(input)?;

// the aove line will be changed to this by ? operator
let sound = match get_sound(input) {
    Ok(value) => value,
    Err(message) => return message,
}
```