fn main() {

    // define enum color
    // #[derive(Debug)]
    enum Color {
        Green,
        Yellow,
        Red,
    }

    // initialize and access enum variants
    let green = Color::Green;
    let yellow = Color::Yellow;
    let red = Color::Red;

    // print enum values
    // println!("{:?}", green);
    // println!("{:?}", yellow);
    // println!("{:?}", red);




    // define enum with
    #[derive(Debug)] // #[derive(Debug)] : allows instances of the type to be formatted using the {:?} placeholder in the println! macro, which is useful for debugging.

    enum Result {
        Score(f64),
    }

    // initialize enum with values
    let num = Result::Score(3.14);
    
    // println!("num = {:?}", num);
}