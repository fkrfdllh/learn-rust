use num_traits::ToPrimitive;

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    // rust can not do calculation between different data type of number

    // let a_f64 = a as f64;
    let a_f64 = a.to_f64().unwrap();

    let c = solve(a_f64, b);

    println!("result of the pythagorean: {}", c);
}

fn solve(a: f64, b: f64) -> f64 {
    return (a.powi(2) + b.powi(2)).sqrt();
}
