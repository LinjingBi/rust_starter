fn main() {
    let num = 45.2/98.6;
    let reminder = 47%8;
    let _t = true;
    let f: bool = false;
    let text = '😻';
    let a = "ded".to_string();
    // let b = a;
    let c = a.clone() + "o";
    println!("{num}");
    println!("{reminder}");
    println!("{f}");
    println!("{text}");
    // println!("{a}");

    // println!("{b}");
    println!("{a}");
    println!("{c}");

    // tuple, immutable
    let s = (1,2,"hello".to_string());
    println!("{}", s.2);

    // array, mutable
    let arr = [1, 2, 3, 4];
    let c = arr[0];
    println!("{c}");
    println!("{:?}", arr);

    let arr_duplicate = [4;5];
    println!("{:?}", arr_duplicate); // [4, 4, 4, 4, 4]

    // default type for integer is i32
    let a: i32 = 8;
    println!("{a}");
    let b: i64 = a.into();
    println!("{b}");

    // default type for float is f64
    let c: f64 = 9.8;
    let d = c as f32;
    println!("{d}");

    let y = {
        let x = 3;
        x + 2  // x + 2 is an expression. Only expression has return value, 
        // and if add semicolon to the end, it will become a statement which does not have return value for y
    };
    println!("{y}");

    let mut i = String::from("uiu"); 
    println!("{i}");
    print_input(&mut i);
    println!("{i}");
    let q = print_input_2(i.clone());
    println!("{q}");
    println!("{i}");
    control_flow();

    let a = 9;
    let b = a;
    println!("{a}"); // fine for number because it is fixed
    println!("{b}");

    let mut a = String::from("hello");
    let d = &a;
    println!("{d}");
    let b = &mut a;
    b.push_str(",ii");
    
    // b.push_str(",ii");
    // println!("{b}");
    
    // println!("{d}");

    let space = find_space(&a);
    println!("space found in {a}, index {space}");

    let a = String::from("test");
    let b: &str = &a[1..]; // &str
    println!("{b}");
    let a = "ytui";
    let b: &str = &a[..3]; // &str
    println!("{b}");

    // structure
    let user1 = User {
        active: true,
        username: "sam".to_string(),
        email: "jduideih@com".to_string(),
        sign_in_count: 1,
    }; // immutable
    println!("{}", user1.email);

    let mut user2 = User {
        active: false,
        username: String::from("pill"),
        email: String::from("dewi9349r@ui.com"),
        sign_in_count: 8,
    }; //mutable
    user2.active = true;
    println!("{}", user2.active);

    let user3 = build_user(String::from("uyt"), String::from("djwuid@heyuwd"));
    println!("{}", user3.sign_in_count);

    let user4 = User {
        email: user1.email,  // we cannot use user1 as whole f.e. let user6 = user1, and also user1.email is also not accessable, because uers1.email is a String, and this part is already freed.
                            // we can still use user1.username, and other fields.
        ..user3 //must come last
        // we cannot use user3 as whole and user3.username, bcs of the String ownership.
        // we can still use user3.email. and user3.active and user3.sign_in_count are also available(bool and u64 are copy, new ownership for user3)
    }; // f.e. if we only use user1.active or sign_in_count here, the whole user1 and each field are still available. all because of the ownership of String, like not copy.
    println!("{}", user4.username);

    let User { active: _x, username: _y, email: _z, sign_in_count: _a } = user4;
    
    // tuple stucture without named fields
    let black = Color(0, 34, 56);
    let origin = Point(1, -3, -5);
    println!("{}", black.1); // 34

    let Point(x, y, z) = origin;
    println!("{x} {y} {z}");
    let Color(x, y, z) = black;
    println!("{x} {y} {z}");

    let r = Rectangle{
        width: 10.3,
        height: 10.7,
    };
    let r2 = Rectangle{
        width: 11.5,
        height: 19.0,
    };
    println!("{:?} area is {}", r, calculate_area(&r));
    println!("{:?} area is {}", r, r.area());
    println!("{:?} can hold {:?}: {}", r, r2, r.can_hold(&r2));
    println!("{:?} is a square.", Rectangle::square(8.0));
    println!("{:?} is 2D: {}", r, Rectangle::is_2d());

    // enum
    let ikind = IpKind::V4;
    let ipdv4 = IpAddr::V4(String::from("2324322"));
    let ipdv6 = IpAddr::V6(127,0,0,1);

    let msg_move = Message::Move{x: 32, y:54};
    let msg_quit = Message::Quit;
    let msg_changecolor = Message::ChangeColor(100, 23, 45);
    let msg_write = Message::Write(String::from("jdwifhe"));

    msg_move.call();

    // Option<T>
    // let mut some_number = Some("op");
    // some_number = Some("iu");
    // some_number = None;


    let coin = Coin::Dime(UsState::Alaska);

    let value = coin.get_value();
    println!("this coin is {value}");

    let result = if let Coin::Dime(_state) = coin {
        if _state.is_aged() {
            Some(String::from("old"))
        } else {
            Some(String::from("young"))
        }

    } else {

        None

    };
    println!("{result:?}");







}

fn print_input(x: &mut String){
    x.push_str("!!!");
}

fn print_input_2(x: String) -> String {
    let y = x + "123";
    y
}

fn control_flow() {
    let a = 8;
    for i in 2..10 {
        if i > a {
            println!("exceed {i}");
            break;
        } else if i == a {
            println!("match");
        } else {
            println!("not there");
        }
    }

    let y = [1,8,2,4];
    let mut index = 0;
    while index < y.len() {
        println!("{}", y[index]);
        index += 1;
    }

}


fn find_space(x: &str) -> usize {
    let strs = x.to_string();

    for (index, item) in strs.chars().enumerate() {
        if item == ',' {
            return index;

        }
    }
    strs.len()
}

// struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(name: String, email: String) -> User {
    User {
        username: name,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn is_2d() -> bool {
        true
    }
}

fn calculate_area(rect: &Rectangle) -> f32 {
    rect.width * rect.height
}

// enum
enum IpKind{
    V4,
    V6,
}

enum IpAddr{
    V4(String),
    V6(u32, u32, u32, u32),
}


enum Message{
    Quit,
    Move {x: u32, y: u32}, // enum can have struct-like variant
    Write(String), 
    ChangeColor(u32, u32, u32), // enum can have tuple-like variant
}

impl Message {
    fn call(&self) {

    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn is_aged(&self) -> bool {
        match self {
            UsState::Alabama => true,
            UsState::Alaska => false,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime(UsState),
    Quarter,
}

impl Coin {
    fn get_value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime(state) => 10,
            Coin::Quarter => 25,
        } // matches must be exhaustive, use _ => xxxx, or other => println!("{other}")
    }
}
