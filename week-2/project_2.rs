fn main() {
    let t:f64 = 450_000.00;
    let m:f64 = 1_500_000.00;
    let h:f64 = 750_000.00;
    let d:f64 = 2_850_000.00;
    let a:f64 = 250_000.00;
    // Sum
    let s = t + m + h + d + a;
    println!("Sum is {}", s);
    // Average
    let avg = (t + m + h + d + a) / 5.0;
    println!("Average is {}", avg);
}