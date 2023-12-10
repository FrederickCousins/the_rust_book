fn main() {
    println!("Welcome... things are about to get... loopy :)");

    let mut i = 0;
    loop {
        if i < 10 {
            println!("again!");
            i += 1
        } else {
            break;
        }
    }
    println!();

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}\n");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}\n");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!\n");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the current value is: {element}");
    }
    println!();

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!\n");
}
