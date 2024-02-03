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