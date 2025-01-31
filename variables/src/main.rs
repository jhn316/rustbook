fn main() {
    //let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    
    let y = y + 1;
    
        {
            let y = y * 2;
            println!("The value of y in the inner scope is: {y}");
        }
    
    println!("The value of y is: {y}");

    let a = 2.0; // f64 by default
    println!("The value of a is: {a}");

    let b: f32 = 3.0; // f32
    println!("The value of b is: {b}");    


    another_function();

}

fn another_function() {
    println!("Another function.");
}
