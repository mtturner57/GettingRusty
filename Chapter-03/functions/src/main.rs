fn five() -> i32 {
    5
}

fn plus_1(x: u16) -> u16 {
    x + 1
}

fn main() {
    let five = five();
    let x = plus_1(5);
    println!("The value of five is: {five}");
    println!("The value of x is: {x}");
} 