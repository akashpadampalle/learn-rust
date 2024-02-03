// // print first and last name
// fn print_full_name(first: &str, last: &str){
//     println!("{} {}", first, last);
// }

// fn sum(num1: i32, num2: i32) -> i32 {
//     num1 + num2
// }

// fn print_num(num: i32) {
//     println!("{:?}", num);
// }

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right
// }

// fn which_way(go: Direction) -> &'static str {
//     match go{
//         Direction::Up => "up",
//         Direction::Down => "down",
//         Direction::Left => "left",
//         Direction::Right => "right",
//     }
// }

// // enum practice-1
// enum Color {
//     Red,
//     Green,
//     Blue,
//     Yellow,
//     Orange,
//     Black,
//     White
// }

// fn print_color(color: Color) {
//     let col = match color {
//         Color::Red => "red",
//         Color::Green => "green",
//         Color::Blue => "blue",
//         Color::Yellow => "yellow",
//         Color::Orange => "orange",
//         Color::Black => "black",
//         Color::White => "white",
//     };
//     println!("{}", col);
// }

// fn main() {
//     print_full_name("John", "Doe");
//     let sum1 = sum(3, 4);
//     print_num(sum1);

//     // if else
//     let is_visiting = false;

//     if is_visiting {
//         println!("hello!");
//     } else {
//         println!("goodbye!");
//     }

//     // if...if else...else
//     let var = 3;

//     if var > 5 {
//         println!(">5");
//     } else if var < 5 {
//         println!("<5");
//     } else {
//         println!("=5");
//     }

//     // match statement
//     match var {
//         v if v > 5 => println!(">5"),
//         v if v < 5 => println!("<5"),
//         5 => println!("=5"),
//         _ => println!("not a valid statement"),
//     }

//     // match statement practice-1
//     let decision = true;

//     match decision {
//         true => println!("it's true"),
//         false => println!("it's false"),
//     }

//     // match statement practice-3
//     let match_var = 3;

//     match match_var {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("other"),
//     }

//     // loop statement practice-1
//     let mut loop_var = 1;

//     loop {
//         println!("{:?}", loop_var);
//         loop_var = loop_var +1;
//         if loop_var > 4 {
//             break;
//         }
//     }

//     println!("loop iteration done");

//     // loop statement practice-2
//     let mut while_var = 5;

//     while while_var > 0 {
//         println!("{:?}", while_var);
//         while_var = while_var -1;
//     }

//     println!("while countdown done!");

//     let way = which_way(Direction::Up);
//     println!("{}",way);

//     print_color(Color::Green);
// }

/* intermediate */
// fn main() {
//     println!("hello intermediate");
//     let drink = Drink {
//         flavor: Flavor::Sweet,
//         oz: 6.0
//     };

//     print_drink(drink);
// }

// enum Flavor {
//     Sparklling,
//     Sweet,
//     Fruity
// }

// struct Drink {
//     flavor: Flavor,
//     oz: f64
// }

// fn print_drink(drink: Drink) {
//     match drink.flavor {
//         Flavor::Sparklling => println!("sparklling"),
//         Flavor::Sweet => println!("sweet"),
//         Flavor::Fruity => println!("fruity"),
//     }
//     println!("{:?}", drink.oz);
// }

/* tuples */

// fn main() {
//     let numbers = (1, 2, 3);
//     let (_x, _y, z) = numbers;

//     println!("{:?}", numbers.0);
//     println!("{:?}", z);

// let (_x_cordinate, y_cordinate) = get_cordinates();

//     let cordinate = get_cordinates();
//     println!("{:?}", cordinate.1);
// }

// fn get_cordinates() -> (f64, f64) {
//     (12.64, 33.22)
// }

/* expression */

// fn main() {
//     let number = 100;
//     let message = if number > 100 {
//         "its big"
//     } else {
//         "its small"
//     };

//     println!("{}", message);

// }

/* owenership */

// struct Grocery {
//     id: i32,
//     quantity: i32
// }

// fn print_grocery_id(grocery: &Grocery) {
//     println!("{:?}", grocery.id);
// }

// fn print_grocery_quantity(grocery: &Grocery) {
//     println!("{:?}", grocery.quantity);
// }

// fn main() {
//     let milk = Grocery {
//         id: 1,
//         quantity: 10
//     };

//     print_grocery_id(&milk);
//     print_grocery_quantity(&milk);
// }

// struct Temprature {
//     deg: f64,
// }

// impl Temprature {
//     fn create(deg: f64) -> Self {
//         Self { deg }
//     }

//     fn print(&self) {
//         println!("{:?}", self.deg);
//     }
// }

// fn main() {
//     let t1 = Temprature::create(64.3);
//     t1.print();
// }

/* impl keyword */

// enum Color {
//     Red,
//     Green,
//     Blue,
// }

// impl Color {
//     fn get_color(&self) -> &str {
//         match self {
//             Color::Red => "red",
//             Color::Green => "green",
//             Color::Blue => "blue",
//         }
//     }
// }

// struct Dimentions {
//     length: f64,
//     height: f64,
//     width: f64,
// }

// impl Dimentions {
//     fn new(length: f64, height: f64, width: f64) -> Self {
//         Self {
//             length,
//             height,
//             width,
//         }
//     }

//     fn print(&self) {
//         println!(
//             "length: {:?}, height: {:?}, width: {:?}",
//             self.length, self.height, self.width
//         );
//     }
// }

// struct Box {
//     dimentions: Dimentions,
//     weight: f64,
//     color: Color,
// }

// impl Box {
//     fn new(dimentions: Dimentions, weight: f64, color: Color) -> Self {
//         Self {
//             dimentions,
//             weight,
//             color,
//         }
//     }

//     fn print(&self) {
//         self.dimentions.print();
//         println!(
//             "weight: {:?}, color: {}",
//             self.weight,
//             self.color.get_color()
//         );
//     }
// }

// fn main() {
//     let b1 = Box::new(Dimentions::new(62.0, 42.1, 11.0), 64.0, Color::Green);
//     b1.print();
//     let v1 = vec![1, 2, 3];
//     let mut v2 = Vec::new();
//     v2.push(1);
// }

// fn main() {
//     let nums = vec![10, 20, 30, 40];

//     for num in &nums {
//         match num {
//             30 => println!("thirty"),
//             _ => println!("{:?}", num)
//         }
//     }

//     println!("total items in nums are: {:?}", nums.len());

// }

/* strings */

// struct Person {
//     name: String,
//     favorite_color: String,
//     age: i32,
// }

// fn print_name(data: &str) {
//     println!("name: {:?}", data);
// }

// fn print_color(data: &str) {
//     println!("favorite color: {:?}", data);
// }

// fn main() {
//     let persons = vec![
//         Person {
//             name: "akash".to_owned(),
//             favorite_color: "yellow".to_owned(),
//             age: 3,
//         },
//         Person {
//             name: "kiran".to_owned(),
//             favorite_color: "black".to_owned(),
//             age: 12,
//         },
//         Person {
//             name: "rango".to_owned(),
//             favorite_color: "orange".to_owned(),
//             age: 10,
//         },
//     ];

//     for person in persons {
//         if person.age >= 10 {
//             print_name(&person.name);
//             print_color(&person.favorite_color);
//         }
//     }
// }
// #[derive(Debug)]
// enum Color {
//     Green,
//     Red,
//     Blue,
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     color: Color,
// }

// fn main() {
//     let p1 = Person {
//         name: String::from("akash"),
//         color: Color::Red,
//     };
//     println!("{:?}", p1);
//     println!("{:?}", Color::Red)
// }

/* advance match */

// enum Ticket {
//     Backstage(i32, String),
//     Vip(i32, String),
//     Standard(i32),
// }

// fn main() {
//     let mut tickets: Vec<Ticket> = Vec::new();
//     tickets.push(Ticket::Backstage(20, "akash".to_owned()));
//     tickets.push(Ticket::Vip(40, "kiran".to_owned()));
//     tickets.push(Ticket::Standard(10));

//     for t in tickets {
//         match t {
//             Ticket::Standard(price) => println!("standard ticket of: {:?}", price),
//             Ticket::Vip(price, name)=> println!("vip ticket of sir {}, price: {:?}", name, price),
//             Ticket::Backstage(price, name)=> println!("Backstage ticket of {}, price: {:?}", name, price),
//         }
//     }
// }

/* option */
// struct Customer {
//     age: Option<i32>,
//     email: String,
// }

// fn main() {
//     let customers: Vec<Customer> = vec![
//         Customer {
//             age: Some(40),
//             email: "akash.padampalle@gmail.com".to_owned(),
//         },
//         Customer {
//             age: None,
//             email: "someone@hotmail.com".to_owned(),
//         },
//     ];

//     for customer in customers {
//         match customer {
//             Customer {
//                 age: Some(value),
//                 email,
//             } => println!("email: {:?}, age: {:?}", email, value),
//             Customer { age: None, email } => println!("email: {:?}", email),
//         }
//     }
// }

// struct Student {
//     name: String,
//     locker: Option<i32>,
// }

// fn main() {
//     let students = vec![
//         Student {
//             name: String::from("akash"),
//             locker: None,
//         },
//         Student {
//             name: "kiran".to_owned(),
//             locker: Some(1),
//         },
//         Student {
//             name: String::from("sheru"),
//             locker: Some(2),
//         },
//     ];

//     for student in students {
//         match student {
//             Student { locker: None, name } => {
//                 println!("{:?} doesn't have locker", name)
//             }
//             Student {
//                 locker: Some(locker_id),
//                 name,
//             } => println!("{:?} have locker id {:?}", name, locker_id),
//         }
//     }
// }

/* documentation */

// /// Services provided by company
// #[derive(Debug)]
// enum Service {
//     Android,
//     Web,
//     IOS
// }

// /// Represent client of company
// #[derive(Debug)]
// struct Client {
//     id: i32,
//     name: String,
//     service: Service,
//     /// is client vip or not
//     vip: Option<bool>,
// }

// /// implenet some additional functionality to Client
// impl Client {
//     /// factory function to create client
//     fn new (id: i32, name: String, service: Service, vip: Option<bool>) -> Self {
//         Self {id, name, service, vip}
//     }
// }

// fn main() {
//     // List of clients
//     let clients = vec![
//         Client::new(1, "wipro".to_owned(), Service::Android, None),
//         Client::new(2, "amazon".to_owned(), Service::Web, Some(true)),
//         Client::new(3, "apple".to_owned(), Service::IOS, Some(true)),
//     ];

//     for client in clients {
//         println!("{:?}", client);
//     }
// }

/* standard liabraries | documentation */
// fn main() {
//     let line = "Lazy Fox Jumps Over The Fench";
//     println!("{:?}", line.to_lowercase());
//     println!("{:?}", line.to_uppercase());
// }

/* Result */
// enum SoundData {
//     Alert,
// }

// fn get_sound(sound: &str) -> Result<SoundData, String> {
//     if sound == "alert" {
//         return Ok(SoundData::Alert);
//     } else {
//         return Err("unable to find sound".to_owned());
//     }
// }

// fn main() {
//     let sound = get_sound("buzz");
//     match sound {
//         Ok(_) => println!("we got sound data"),
//         Err(message) => println!("{:?}", message),
//     }
// }

// #[derive(Debug)]
// enum MenuChoice {
//     MainMenu,
//     Start,
//     Quit,
// }

// fn get_choice(input: &str) -> Result<MenuChoice, String> {
//     match input {
//         "mainmenu" => Ok(MenuChoice::MainMenu),
//         "start" => Ok(MenuChoice::Start),
//         "quit" => Ok(MenuChoice::Quit),
//         _ => Err("Unable to find".to_owned()),
//     }
// }

// fn print_choice(choice: &MenuChoice) {
//     print!("choice = {:?}", choice);
// }

// fn main() {
//     let choice: Result<MenuChoice, String> = get_choice("start");

//     match choice {
//         Ok(inner_choice) => print_choice(&inner_choice),
//         Err(message) => println!("{:?}", message),
//     }

// print_choice(&choice)
// println!("{:?}", choice);
// match choice {
//     Ok(_) => println!("valid choice"),
//     Err(message) => print!("{:?}", message),
// }
// }

/* result */

// #[derive(Debug)]
// struct Adult {
//     name: String,
//     age: i32,
// }

// impl Adult {
//     fn new(name: String, age: i32) -> Result<Self, String> {
//         if age > 20 {
//             Ok(Self { name, age })
//         } else {
//             Err("Age is less than 21".to_owned())
//         }
//     }
// }

// fn main() {
//     let young = Adult::new("vishu".to_owned(), 20);
//     let adult = Adult::new("akash".to_owned(), 23);

//     let persons = vec![young, adult];

//     for p in persons {
//         match p {
//             Ok(Adult { name, .. }) => println!("{:?} let's have drink", name),
//             Err(message) => println!("{}", message),
//         }
//     }
// }

/* result and question mark operator */