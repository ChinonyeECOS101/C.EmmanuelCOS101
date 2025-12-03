fn main() {
    let chi = vec![101,250, 330, 400];
    //vectors v own the object in heap

    //only a single variable owns the heap memory at any given time

    let ama = chi;
    //here two variables owns heap value,
    //two pointers to the same content is no allowed in rust 

    /*Rust is vary smart in terms of memory access, so it detects a race
    condition as two varaibles point to same heap*/

    println!("{:?}",ama );
}
