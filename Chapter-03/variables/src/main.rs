const THREE_HOURS_IN_SECONDS: u32 = 60 *60 * 3;

fn main() {
    variables();
    shadowing();
    operation();
    characters();
}

fn variables()  {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of the const is: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn operation() {
        // addition
        let sum = 5 + 10;
        println!("Sum: {sum}");

        // subtraction
        let difference = 95.5 - 4.3;
        println!("Difference: {difference}");
    
        // multiplication
        let product = 4 * 30;
        println!("Product: {product}");
    
        // division
        let quotient = 56.7 / 32.2;
        println!("Quotient: {quotient}");

        let quotient = -5.0 / 3.0;
        println!("Quotient2: {quotient}");

        let truncated = -5 / 3; // Results in -1
        println!("Truncated: {truncated}");
    
        // remainder
        let remainder = 43 % 5;
        println!("Remainder: {remainder}");
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation

    let heart_eyed_cat = '😻';
    println!("Heart Cat: {heart_eyed_cat}");
}
