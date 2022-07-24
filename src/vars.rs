// Variable hold primitive data or references to data
//Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Gui"; //Immutable
    let mut age = 25; //Mutable

    println!("My name is {} and i have {}",name,age);

    //Constants (Need to define datatype)
    const ID: i64 = 585205;
    println!("ID:{}",ID);

    //Multiples Vars
    let (my_name,age) = ("Gui",25);
    println!("{} is {}",my_name,age)
}