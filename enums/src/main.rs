
/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}


fn main() {

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }


}
