use rshttp::client::*;

fn main() {
    println!("{:?}", get("example.com:80"))
}
