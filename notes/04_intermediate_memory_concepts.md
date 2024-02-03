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

