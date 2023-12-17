fn main() {
    let _s = "initial contents".to_string();
    // is equivalent to
    let s = String::from("inital contents");
    let b = " bonus contents";

    // we can concatenate with '+'
    let new = s + b;
    println!("{new}");

    // we can also use 'push_str' to grow a mutable string
    let mut s = "foo".to_string();
    s.push_str("bar");
    println!("{s}");

    // the push operator works the same but with characters only
    s.push('!');
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // if we wanted to form tic-tac-toe, we could use '+' but it does become unwieldy
    // Instead we can use the format! macro

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    // Easier!

    // YOU CANNOT USE INDEXING ON STRINGS
    // utf-8 does not prescribe a fixed byte length per character
    //
    // eg. how many bytes is "नमस्ते"?
    // it is stored like this:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    //  224, 165, 135]
    // thus 18 bytes
    // even still! there are diacritics which are not characters by themselves.
    //
    // so to iterate over the chars in a list we can use:

    let s = "Rust".to_string();
    for c in s.chars() {
        println!("{c}");
    }

    // and if you do want the bytes:

    for b in s.bytes() {
        println!("{b}");
    }
}
