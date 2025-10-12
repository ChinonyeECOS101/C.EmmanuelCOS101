fn main() {
	//Declear the ammount forach item
	let toshiba:f32 = 450_000.0;
	let mac:f32 = 1_500_000.0;
	let hp:f32 = 750_000.0;
	let dell:f32 = 2_850_000.0;
	let acer:f32 = 250_000.0;

	// Calculate total sum of sales 
	let noni:f32 = toshiba + mac + hp + dell + acer;
	
	// Caculate average sum of sales 
	let empire:f32 = noni / 5.0;

	// Display results

	println!("__________________________________________________________________");
	println!("SALE RECORD ");
	println!("__________________________________________________________________");
    println!("TOTAL SALES AMOUNT = {}", noni);
    println!("AVERAGE SALES AMOUNT ={}", empire);

}