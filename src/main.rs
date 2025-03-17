use num_traits::{Float, ToPrimitive};

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;
    let c = solve::<f32, f64>(a, b);

    println!("result of the pythagorean: {}", c);
}

fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a64 = a.to_f64().unwrap();
    let b64 = b.to_f64().unwrap();

    return (a64.powi(2) + b64.powi(2)).sqrt();
}
