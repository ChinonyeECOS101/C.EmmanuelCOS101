fn main() {

    use std::io;

    let mut food_orders = String::new();
    let mut quantity_of_food = String::new();

    println!("WELCOME TO NONI_EMPIRE RESTAURANT");
    println!("");
    println!("");

    println!("List Of Meal Available And Their Price");
    println!("");
    println!("Pounded Yam/Edinkaiko Soup - ₦3_200 ");
    println!("Fried Rice & Chicken - ₦3_000 ");
    println!("Amala & Ewedu Soup - ₦2_500");
    println!("Eba & Egusi Sou - ₦2_000");
    println!("White Rice & Stew - ₦2_500");


    //Customers orders entery

    println!("Enter Your Order here");
    io::stdin().read_line(&mut food_orders).expect("Not a valid String");
    let foodi:&str = food_orders.trim();
    println!("");
    println!("");


    //prompt of food quantity
    println!("Enter Your Desired Quantity");
    io::stdin().read_line(&mut quantity_of_food).expect("Not a valid String");
    let quan:i32 = quantity_of_food.trim().parse().expect("Quantity Error");

    //Matching String Slice


    let (foodi, price) = match foodi
    {
        "pondo yam/edinkaiko soup" =>("pondo yam/edinkaiko soup", 3_500),
        "fried rice & chicken" =>("fried rice & chicken", 3_000),
        "amala & ewedu soup" =>("amala & ewedu soup", 2_500),
        "eba & egusi soup" =>("eba & egusi soup", 2_000),
        "white rice & stew" =>("white rice & stew", 2_500),
        _=>{
            println!("Order Error, Please input a valid order");
            return;
        }
    };

    //Customer order breakdown

    let mount:i32 = price * quan;
    println!("Your Order: {}",foodi);
    println!("Price per plate: ₦{}",price);
    println!("Quantity: ₦{}",quan);
    println!("Amount: ₦{}",mount );

    // Calculation of our resturant promo above ₦100_000

    let overall_total = if mount > 100_000{
        let promo = mount as f32 * 0.05;
        println!("Discount applied: 5% (₦{:.2})",promo );
        (mount as f32 - promo) as i32
    }else {
        println!("Not qualified for a discount");
        mount
    };

    println!("Total Amount:₦{}",overall_total );
    println!("");
    println!("Thanks For Your Patronage, Looking Forward To Serve You again.");






}
