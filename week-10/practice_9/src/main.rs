// define dimensions of a rectangle 

struct Rectangle {
    width:u32, hei:u32
}

//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u32 {
        // use the . operator to fetch the valu of a field via the self keyword
        self.width * self.hei
    }

}
fn main() {
    // instanatiated the structure
    let small = Rectangle {
        width:10,
        hei:20
    };
    // print the rectangle's area
    println!("width is {} \n height is {} \n area of Rectangle
        is {}",small.width,small.hei,small.area() );
}