enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    /*
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));
    */


    let absent_number: Option<i32> = None;
    dbg!(absent_number);
}
/*
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
 */
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    dbg!(sq);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
        println!("The rectangle has a nonzero width; it is {}", rect.height);
    }

}
 */
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
 */
/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area0(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

fn area_1(rectagle: &Rectangle)-> u16{
    rectagle.width * rectagle.height
}

fn main() {

    let scale = 2;

    let rect1 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {rect1:#?}");
        println!(
        "The area of the rectangle is {} square pixels.",
        area_1(&rect1)
    );

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area0(rect1)
    );
}
*/

/*struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Inches(i32);
//struct Centimeters(i32);

fn print_length(len: Inches){
    println!("{} in", len.0)
}

struct AlwaysEqual;

fn main() {

    let subject = AlwaysEqual;
    dbg!(subject);

    let x = Inches(10);
    print_length(x);
    // First user created
    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };
    print_user(&user1);

    // Same user but with a different email
    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);

    // User 2 created
    let user2 = build_user("a@a.com".to_string(), "user124".to_string());
    print_user(&user2);

    //* User 3 created copying user1
    let user3 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count,
    };
    print_user(&user3);
    // */

    // User 4 created copying user1 but using a different syntax
    let user4 = User{
        email: String::from("yetanother@one.com"),
        ..user2
    };
    print_user(&user4);

}

fn print_user(user: &User) {
    println!("{}", user.active);
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
}*/
