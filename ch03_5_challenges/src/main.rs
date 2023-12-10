use ordinal::Ordinal;

fn main() {
    let x: f64 = -40.;
    println!("{x} C = {} F", celsius_to_fahrenheit(x));
    println!("{x} F = {} C\n", fahrenheit_to_celsius(x));

    let n: i64 = 40;
    println!("Fib({n}) = {}\n", fib(n));

    twelve_days_of_xmas();
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (1.8 * celsius) + 32.
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) * 5. / 9.
}

fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn twelve_days_of_xmas() {
    let gifts: [&str; 12] = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            Ordinal(i).to_string()
        );
        if i == 1 {
            println!("A partridge in a pear tree\n")
        } else {
            for j in (0..i).rev() {
                println!("{}", gifts[j].to_string());
            }
            println!();
        }
    }
}
