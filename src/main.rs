fn main() {
    println!("! Hello, world!");
    println!("global:{}", std::env::var("APP_NAME").unwrap());
    println!("local:{}", std::env::var("LOCAL").unwrap());
}
