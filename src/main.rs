use std::io::Error;

fn main() {
    match divide(5.0, 0.0) {
        Ok(number) => println!("{}", number),
        Err(err) => println!("{}", err),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error::other("can't divide by 0"));
    }

    return Ok(a / b);
}
