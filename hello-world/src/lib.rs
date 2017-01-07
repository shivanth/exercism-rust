
pub fn hello(name : Option<& str>) -> String{
    let greeting = match name{
        Some( ref x) => format!("Hello, {}!", x),
        None => "Hello, World!".to_owned(),
    };
    greeting
}
