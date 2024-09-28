fn main() {
    let number = 3;

    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    let x = is_divisable(number);

    println!("{}",x);
    
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");
}

fn is_divisable(x: u32) -> String {
    if x % 4 == 0 {
        "number is divisible by 4".to_string()
    } else if x % 3 == 0 {
        "number is divisible by 3".to_string()
    } else if x % 2 == 0 {
        "number is divisible by 2".to_string()
    } else {
        "number is not divisible by 4, 3, or 2".to_string()
    }
}
