fn main() {
	let ada:f32 = 510_000.0;
	let obi:f32 = 5.0;
    let fra:f32 = 3.0;

    // value of the TV
    let a = ada * (1.0 - (obi / 100.0)).powf(fra);
    println!("Value of the TV {}", a);

}