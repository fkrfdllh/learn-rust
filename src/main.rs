use num_traits::{Float, ToPrimitive};

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;

    // ::<f32> gonna replace <T> with <f32>
    let c = solve::<f32>(a, b);

    // not using ::<f32> gonna runs well too
    // let c = solve(a, b);

    println!("result of the pythagorean: {}", c);
}

fn solve<T: Float>(a: T, b: T) -> f64 {
    let a64 = a.to_f64().unwrap();
    let b64 = b.to_f64().unwrap();

    return (a64.powi(2) + b64.powi(2)).sqrt();
}
