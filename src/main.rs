mod scanner;
fn main() {
    let tokens = scanner::scan(String::from("input"));
    match tokens {
        Ok(x) => println!("{:#?}", x),
        Err(err) => println!("{}", err),
    }
}
