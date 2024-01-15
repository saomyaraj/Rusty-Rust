#[allow(unused_variables)] //This will allow the unused variables and suppress the warning.

fn main() {
    greet();

    let mut x: i32 = 5; //Mutating x so that it can be changed later. But we can instantiate/Override it with a new value.
    let _y: i32; //Uninitialized but also unused, that's why used _ before variable to prevent error.
    assert_eq!(x,5); //Like if statement, if x is equal to 5 then executes further, otherwise throws error.
    println!("The earlier version of x is {}",x);
    x = x + 2;
    println!("The later version of x is {}", x); //Format string to print the value of x.

    let (a, b) = (1,2);
    println!("The value of a is {} and the value of b is {}", a, b);

    let c1: char = 'a'; //Single apostrophe for character and double for string
    print_char(c1);

    let t: bool = true;
    if !t {
        println!("Will not be printed because t is false");
    }

    println!("Success!");
}
fn greet() {
    let x: &str = "Hello";
    println!("{}, World",x);
}

fn print_char(c: char) {
    println!("{}", c);
}