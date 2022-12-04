use std::io::{stdin, stdout, Write, Read};
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
            println!("{}", err_msg);
            c = get::<T>(prompt, err_msg);
        },
    }
    return c;
}

pub fn read_file_from_stdin() -> String {

        let _ = stdout().flush().expect("Console flush failed.");

        let mut buffer:String = String::new();
        let _ = stdin().read_to_string(&mut buffer);

        buffer.trim().to_string()
                        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_std () { // enter "sdf", newline, "sdf", newline, EOF
        assert_eq!(read_file_from_stdin(), "sdf\nsdf")
    }
}



