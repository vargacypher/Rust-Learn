/*
Primitive --
Int: u8,i8,u16,i16,u32,i32,u64,i64.... (number of bits taken in memory)
Floats: f32, f64
Bool
Characters (char)
Tuples
Arrays


*/

/* Rust is a statically lang, so it must know the types 
of all vars at compile time, however, the cimpile can 
ussually infer what types we want to use based on the values.
*/

pub fn run() {

    //Default is "i32"
    let x = 1 ;

    //Default if "f64"
    let x = 2.5 ;

    //Defining type
    let y: i8 = 1;

    //Finding max size of a type
    println!("Max i32: {}", std::i32::MAX); //std is a library


}