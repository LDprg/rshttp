use rshttp::client::*;

fn main() {
    println!("{}", get("https://example.com").unwrap())
}
