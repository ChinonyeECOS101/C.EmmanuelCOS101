use std::io::Write;


    //constructing the student management information system format.
struct Details {
        name:String,
        matric:String,
        department:String,
        level:u32,
    }


fn main() {

    //storing students data using vector
    let students = vec![
        Details {
         name:"Oluchi Mordi".to_string(),
         matric:"ACC1021111".to_string(),
         department:"Accounting".to_string(),
         level:300,
        },
        Details {
            name:"Adams Aliyu".to_string(),
            matric:"ECO10110101".to_string(),
            department:"Economics".to_string(),
            level:100,
        },
        Details {
            name:"Shenia Bolade".to_string(),
            matric:"CSC10328828".to_string(),
            department:"Computer".to_string(),
            level:200,
        },
        Details {
            name:"Adekunle Gold".to_string(),
            matric:"EEE11020202".to_string(),
            department:"Electrical".to_string(),
            level:200,
         },
        Details {
            name:"Blanca Edemoh".to_string(),
            matric:"MEE10202001".to_string(),
            department:"Mechanical".to_string(),
            level:100,
        },
    ];


    //header output
    println!("{:<20} {:<20} {:<20} {:<10}",
    "Student Name", "Matric. Number", "Department", "Level");

    //student deta+ils output
    for a in &students{
        println!("{:<20} {:<20} {:<20} {:<10}",
       a.name, a.matric, a.department, a.level );
    }

    //creating student file

    let mut noni = std::fs::File::create("My_student.txt").expect("error");
    file.write_all("DETAILS OF PAU STUDENTS\n"
        .as_bytes())?.expect("failed");


    // writing the header to file
    let header = format!("{:<20} {:<20} {:<20} {:<10}",
    "Student Name", "Matric. Number", "Department", "Level"
    );
    file.write_all(header.as_bytes())?.expect("Error"); 

    // writing students details into the file

    for a in &students {
        let add = format!("{:<20} {:<20} {:<20} {:<10}",
    a.name, a.matric, a.department, a.level );
        file.write_all(add.as_bytes())?.expect("Error");
    } 




    println!("\nStudent File is created sucessfully");



}