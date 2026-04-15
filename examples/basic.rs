fn main() {
    let path = zero_copy_env::get("PATH");

    println!("PATH len: {:?}", path.map(|v| v.len()));

    let port: u16 = zero_copy_env::parse_env("PORT").unwrap_or(8080);

    println!("PORT: {}", port);
}
