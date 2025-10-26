use rshttp::client::*;

fn main() {
    println!("{}", get("http://example.com").unwrap())
}
