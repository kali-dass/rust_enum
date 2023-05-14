//create enum
enum IpAddrKind {
    V4,
    V6
}
fn main() {
    println!("Hello, world!");

    // use enum values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
