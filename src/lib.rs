use std::io::{stdin, stdout, Write};
use std::str::FromStr;


fn get_str(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = stdout().flush().expect("Console flush failed.");

    let mut buffer:String = String::new();

    // `read_line` returns `Result` of bytes read
    let _ = stdin().read_line(&mut buffer).expect("Reading line failed.");
    return buffer.trim().to_string();
}

pub fn get<T: FromStr>(prompt: &str, err_msg: &str) -> T {
    let i: String = get_str(prompt);
    let res = i.parse::<T>();
    let c;
    match res {
        Ok(a) => c = a,
        Err(_) =>{
            println!("{}", err_promt);
            c = get::<T>(prompt, err_promt);
        },
    }
    return c;
}

