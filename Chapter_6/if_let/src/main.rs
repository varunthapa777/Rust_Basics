fn main() {
    let config_max: Option<u8> = None;

    if let Some(max) = config_max {
        println!("Config max = {}", max);
    } else {
        println!("Value is None");
    }
}
