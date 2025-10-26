use rshttp::client::*;

fn main() {
    println!("{:?}", get("google.com:80"))
}
