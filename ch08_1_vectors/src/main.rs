fn main() {
    // This works:
    // let v: Vec<i32> = Vec::new();

    // But this is better:
    let v: Vec<i32> = vec![1, 2, 3];
    // What's going on here?
    // 1.  We are using the vec! macro to initialise a vector
    // 2. The vec! macro knows that we mean for our vector values
    //    to have type i32 since those are the values that we
    //    have given it.
    //    Note that this would NOT work had we given values of different types

    println!("{:?}", v);

    // Remember rust is zero-indexed and thus to get the third element of an array
    // we use index 2

    let third = &v[2];
    println!("The third element of array v is: {third}");

    // We can also use the 'get' method, which is useful because it returns an option
    // Thus, should we have gone out of bounds, we don't have to crash the program.

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // TEST
    // What is wrong with this code?
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {first}");

    // We hold an immutable reference 'first' and then try to take a mutable reference
    // in the process of pushing a value
    // This could be unsafe as the push may cause the vector to exceed its maxsize.

    // ITERATING OVER A VECTOR
    // can be done like so

    let v = vec![100, 200, 150];
    for i in v {
        println!("{i}");
    }

    // We can also take mutable references to moidfy values in places

    let mut v = vec![100, 200, 150];
    for i in &mut v {
        *i /= 50;
        println!("{i}")
    }
    // Pretty cool!
    // * note the dereferencing operation going on
    // wait for chapter 15 for more on this


    // How might you approach storing values of different types in a vector?
    // The trick is to use an enum

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Float(45.),
        SpreadsheetCell::Text(String::from("value")),
    ];

    println!("{:?}", row)
}
