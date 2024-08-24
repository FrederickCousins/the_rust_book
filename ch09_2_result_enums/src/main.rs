use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // This is one approach to handling a fail to open file error
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Failed to open file with error code {error}"),
    // };

    // However, we can better handle the error varieties
    // e.g. if the file doesn't exist maybe we want to create it!

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Some other error occurred: {:?}", other_error),
        },
    };
}
