use std::fs::File;
use std::io::ErrorKind;
use std::io::Read; // bring the Read trait into scope (for .read_to_string())

// two types of errors: unrecoverable: panic immediately. recoverable: return Result<T, E> and let outer layer to handle error.

fn main() {
    let file_name = "create_0.txt";

    // A. match..match..match..
    // match File::open(file_name) {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create(file_name) {
    //             Ok(file) => {
    //                 println!("created a new {file_name}.");
    //                 file
    //             },
    //             Err(er) => {
    //                 panic!("{file_name} not found and also failed to create a new one: {er:?}")
    //             },
    //         },
    //         _ => panic!("failed to open {file_name}, error: {err:?}"),
    //     },
    // };

    // B. unwrap_or_else + if else
    // File::open(file_name).unwrap_or_else(|error|{
    //     if error.kind() == ErrorKind::NotFound {
    //         println!("failed to find {file_name}, creating one...");
    //         File::create(file_name).unwrap_or_else(|err|{
    //             panic!("failed to create {file_name}, error {err:?}");
    //         })
    //     } else {
    //         panic!("failed to open {file_name}, error: {error:?}");
    //     }
    // }
    // );

    // C. unwrap and expect: return the type T in Result<T, E> and panic if E.
    // File::open(file_name).unwrap(); // work but no customized error msg
    let file = File::open(file_name).expect("failed to open {file_name}"); // good for prod env. panic with customized msg and error details
    println!("{file:?}");

    let _ = get_file_content(file_name);

}

fn get_file_content(file_name: &str) -> Result<String, std::io::Error> {
    // A. use match step by step
    // let mut file = match File::open(file_name) {
    //     Ok(fc) => fc,
    //     Err(error) => return Err(error), 
    // };
    // let mut content = String::new();
    // match file.read_to_string(&mut content) {
    //     Ok(_) => {
    //         println!("content: {content}");
    //         Ok(content)
    //     },
    //     Err(err) => Err(err),
    // }
    // B. use ? when a function return Result, Option, or another type that implements FromResidual
    let mut file = File::open(file_name)?; // propagate if error
    let mut content = String::new();
    file.read_to_string(&mut content)?; // <-- Result<usize, Error>, use uszie when you only care how many bytes were read (not the contents)
    Ok(content) // Result<String>
}

