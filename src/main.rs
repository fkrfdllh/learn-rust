fn main() {
    let a: f32 = 3.0;
    let b = 4.0;
    let c = solve(a, b);

    println!("result of the pythagorean: {}", c);
}

fn solve(a: f64, b: f64) -> f64 {
    return (a.powi(2) + b.powi(2)).sqrt();
}
