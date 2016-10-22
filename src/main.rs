// Printing in Rust

fn main() {
    println!("I'm at the second Rustbridge ever!");
    let language = "Rust";
    println!("Gosh, I like {} already!", language);
}

// Functions

fn add_fifty(n: i32) -> i32 {
    n + 50
}
fn main() {
    println!("Lots: {}", add_fifty(100));
}

// Conditionals: if

fn main() {
    let age = 13u32;
    if age < 13 {
        println!("You may see G or PG movies");
    } else if age < 17 {
        println!("You may see G, PG, or PG-13 movies");
    } else {
        println!("You are old.");
        println!("You may see G, PG, PG-13, or R movies");
    }
}

// Conditionals: match
// ... means inclusive range

fn main() {
    let age = 13u32;
    match age {
        0...12 => println!("You may see G or PG movies"),
        13...16 => println!("You may see G, PG, or PG-13 movies"),
        _ => {
            println!("You are old.");
            println!("You may see G, PG, PG-13, or R movies")
        },
    }
}

// Arrays: Kinda like arrays in other languages, but worse.
// {:#?} = pretty debug

fn main () {
    let mut color = [255, 0, 255];
    color[0] = 100;
    println!("The color is {:#?}", color);
}

// Panic stops your program with a message.

fn main() {
    panic!("aaaaa!");
}

// Rust is very careful about memory. Will panic below.

fn main () {
    let color = [255, 0, 255];
    let index = 9;
    println!("The 10th element is {:?}", color[index]);
}

// Vectors, must add mut to modify/add items to the vector

fn main () {
    let mut prices = vec![30, 100, 2];
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:?}", prices);
}

// Looping (and ranges)

fn main () {
    let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
    for name in names.iter() {
        println!("Hi {}!", name);
    }
}

// Iterators

fn main () {
    // Filters = keep!

    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }

    // Map = computation

    for i in (0..10).map(|x| x * x ) {
        println!("i = {}", i);
    }

    // Fold = reduce

    let sum = (0..10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);
}

// Enumerators

fn main () {
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    let light = TrafficLight::Green;
    match light {
        TrafficLight::Red => println!("STOP!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go go go!"),
    }
}

// Emums

fn main () {
    enum GameType {
        SinglePlayer,
        MultiPlayer(u32),
    }
    let game = GameType::MultiPlayer(4);
    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => {
            println!("How about {}-player tag?", num)
        },
    }
}

// Options

fn main () {
    let b: Option<&str> = None;
    match b {
        Some(name) => {
            println!("Other name is {} bytes long", name.len())
        },
        None => {
            println!("No name!")
        }
    }
}

// Results, like Option but for failures
// Rust doesn't have exceptions; using Result is how you handle errors.

fn main () {
    let numstr = "florp";
    // parse = grab a number
    let num = numstr.parse::<i32>();
    let answer = match num {
        Ok(n) => n + 5,
        Err(_) => 0,
    };
    println!("Answer is {}", answer);
}

// try! macro to propagate errors up

fn main () {
    fn add_five_to_string(s: String) ->
        Result<i32, std::num::ParseIntError> {
        let ans = try!(s.parse::<i32>()) + 5;
        Ok(ans)
    }
}

// String slices

fn main () {
    let v = vec![1, 2, 3, 4, 5];
    let piece = &v[3..];
    println!("piece of v = {:?}", piece);
}

// Rust Ranges

// START..END
// START..
// ..END
// ..

// String Slices

fn main () {
    let s = String::from("Call me Ishmael blah blah...");
    let part = &s[0..4];
    println!("part is '{}'", part);
}

// Ownership

fn main() {
    let v = vec![1, 2, 3];
    println!("v is valid here! {:?}", v);
}

// Tranferring Ownership

fn main() {
    let v = vec![1, 2, 3];
    print_vec(v);
    // print once, then panics since the ownership is given away
    print_vec(v);
}

fn print_vec(v: Vec<i32>) {
    println!("v is {:?}", v);
}

// References

fn main() {
    let v = vec![1, 2, 3];
    print_vec(&v[..]);
    print_vec(&v[..]);
}
fn print_vec(v: &[i32]) {
    println!("v is {:?}", v);
}

// Mutable References

fn main() {
    let mut v = vec![1, 2, 3];
    change_vec(&mut v[..]);
    change_vec(&mut v[..]);
    println!("v is {:?}", v);
}
fn change_vec(v: &mut [i32]) {
    v[0] *= 5;
}

// Not allowed to have mutable and immutable references in the same scope.

fn main() {
    let mut v = vec![1, 2, 3];
    let f = &v[0];
    v.clear();
    println!("What would f be? {}", f);
}
