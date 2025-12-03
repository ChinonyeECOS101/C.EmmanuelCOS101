use std::io::Write;


fn main() {

    // Drinks avaliable
    let d1:&str = "
    NIGERIAN BREWERY DRINKS COMPANY

    LAGER:
    33 Export
    Desperados
    Goldberg
    Gulder
    Heineken
    Star
    ";

    let d2:&str = "
    STOUT:
    Legend
    Turbo King
    Williams
    ";

    let d3 = "
    NON-ALCOHOLIC:
    Amstel Malta
    Malta Gold
    Fayorouz
    ";

    let mut file = std::fs::File::create("noni.txt").expect("create failed");
    file.write_all("DRINKS AVALIABLE\n"
        .as_bytes()).expect("failed");
    file.write_all(d1.as_bytes()).expect("Error");
    file.write_all(d2.as_bytes()).expect("Error");
    file.write_all(d3.as_bytes()).expect("Error");



    println!("\nWELCOME TO NIGERIAN BREWERY LIMITED");
}
