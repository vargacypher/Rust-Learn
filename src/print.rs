pub fn run() {
    
    //Noamla print
    println!("Hello!!!");
    
    //Formatter
    println!(
        "{} is from {}", 
        "Gui", "Brazil"
    );
    //or positional arguments
    println!(
        "{0} is from {1}",
        "Gui","Brazil"
    );

    //or positional arguments
    println!(
        "{name} is from {country}",
        name = "Gui",country="Brazil"
    );

}