# rtext - A simple library to read input from the Console in Rust

I created this library to read input from the  Console in Rust customized to my needs.

## Installation

Just add the following line to your `Cargo.toml`

`rtext = {git = "https://github.com/dgc08/rtext-lib"}`

## Use of the library

The library contains two functions:

 `rtext::get<T: FromStr>(prompt: &str, err_msg: &str) -> T ` <br>
 `rtext::get_file_from_stdin() -> String `

### rtext::get

It is used to read one line from`stdin` to `String`, and then parse it into the Type `T`.

#### Function signature

`rtext::get` takes two arguments: The prompt and the Message, which is printed if the parse fails.

##### Read as String

`let my_string = rtext::get::<String>("Please enter a String: ", "");`
<br><br>


The `::<String>` is not needed if the compiler knows to which type to cast:

`let my_string:String = rtext::get("Please enter a String: ", "");`

<br>

An error Message is not needed because theres nothing that could go wrong while casting a String to a String.

#### Read as any other type

The function can also parse to any type implementing the `FromStr` trait. Builtin-types like `float` and `int` do this, but be careful with specifing own types.

You have to specify an error Message, because an  `ParseError` may occur.

##### Examples:

###### Integer:

`let my_integer = rtext::get::<i32>("Please enter a number: ", "Sorry, that doesn't look like an i32. Please try again!");`

###### Float:

`let my_float = rtext::get::<f64>("Please enter a float: ", "Sorry, that doesn't look like an f64. Please try again!");`

### rtext::read_file_from_stdin

The function will recive input from `stdin` until it gets an `EOF`(On WIndows `CTRL`+`Z`, On Linux `CTRL`+`D` ). This is useful for piping files into your program, e.g.
`cat example.txt | yourprogram`
, or reading multiple lines from the console. 

#### Use of rtext::read_file_from_stdin

`let file_content = rtext::read_file_from_stdin();`

`file_content` now contains the lines read from `stdin`
