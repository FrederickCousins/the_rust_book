fn main() {
    println!("Hello, world!");
    
    another_function(5);
    
    print_labelled_measurement(5, 'h');
    
    let x = five();
    println!("The value of x is {x}");

    let x = plus_one(10);
    println!("The value of x is {x}");
}

fn another_function(x: i32) {
    println!("What's this? Another function appears");
    println!("The value of x is {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
