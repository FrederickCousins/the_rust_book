fn main() {
    let number = 7;

    if number < 5 {
        println!("True: number < 5");
    } else {
        println!("False: number >= 5");
    }

    if number != 0 {
        println!("number was not zero")
    } else {
        println!("number was zero")
    }

    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else {
        println!("number not divisible by 4 or 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("le valuee de number est {number}");
}
