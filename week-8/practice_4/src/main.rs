fn main() {
    //Name vector 

    let name = vec!["mary", "Hola", "baddie", "pola", "rammy", "tasha"];

    //age vector
    let age = vec![16, 17,19, 22,20,21,18,23];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector 
    for i in 0..age.len()
    {
        // iterating through i on thte vector 
        print!("{} is {} years old\n",name[i],age[i] );
    }
}
