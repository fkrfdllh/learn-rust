use num_traits::ToPrimitive;

fn main() {
    let a: u32 = 3;
    let b: f64 = 4.0;
    let c = solve(a, b);

    println!("result of the pythagorean: {}", c);
}

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a64 = a.to_f64().unwrap();
    let b64 = b.to_f64().unwrap();

    return (a64.powi(2) + b64.powi(2)).sqrt();
}
