use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your weight (kg):");
    io::stdin().read_line(&mut input).unwrap();
    let weight = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    print!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}
