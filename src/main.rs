fn main() {
    println!("Hello, world!");


    structs();

    enums();
}

fn structs() {
    struct Pair(i32, i32);

    struct Rectangle {
        top_left: Pair,
        bottom_right: Pair,
    }

    let rect: Rectangle = Rectangle{top_left: Pair(1,2), bottom_right: Pair(3,4)};

    let Rectangle { top_left: Pair{0: x, 1: y}, bottom_right: right}  = rect; 

    print!("{} {} ", x, y);

}

enum Event {
    PageX,
    Key(String)
}

impl Event {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::PageX => x + y,
            _ => 0
        }
    }
}

fn enums() {
    fn inpsect(event: Event) {
        match event {
            Event::Key(st) => println!("{}", st),
            _ => println!("missing")
        }
    }

    type XMen = Event;

    inpsect(Event::Key(String::from("EVENT_KEY1")));
    inpsect(XMen::Key("EVENT_KEY2".to_owned()));
    inpsect(XMen::PageX);

    println!("{}", XMen::PageX.run(3, 4));

    let s1 = "123".to_owned();
    let s2 = String::from("456");
    let s3 = "789";

    let s4 = s1 + &s2 + s3;
    print!("{}",  s4);

}
