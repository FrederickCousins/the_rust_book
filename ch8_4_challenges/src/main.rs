mod challenge1;
mod challenge2;
mod challenge3;

use challenge1::{median, mode};
use challenge2::pig_latin;
use challenge3::run;

fn main() {
    // Challenge 1:
    // Given a list of integers, use a vector and return the median (when
    // sorted, the value in the middle position) and mode (the value that
    // occurs most often; a hash map will be helpful here) of the list.

    let v = vec![
        1, 6, 5, 3, 6, 12, 7, 4, 6, 2, 4, 547, 34, 3, 5, 3, 23, 23, 4, 6, 7, 3, 2,
    ];

    let result = median(&v);

    match result {
        Some(median) => println!("Median: {}", median),
        None => println!("The list is empty."),
    }

    let result = mode(&v);

    match result {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("The list is empty."),
    }

    // Challenge 2
    // Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    let s1 = String::from("together");
    let s2 = String::from("allow");

    let result = pig_latin(&s1);
    match result {
        Some(res) => println!("{res}"),
        None => println!("The string is empty."),
    }

    let result = pig_latin(&s2);
    match result {
        Some(res) => println!("{res}"),
        None => println!("The string is empty."),
    }

    // Challenge 3
    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example, “Add Sally
    // to Engineering” or “Add Amir to Sales.” Then let the user retrieve a
    // list of all people in a department or all people in the company by
    // department, sorted alphabetically.

    run()

    


}
