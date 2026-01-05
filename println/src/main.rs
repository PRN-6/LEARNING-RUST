fn main() {

//   let name = "prinson";
//   println!("Hello, {}",name)


    let mut x = 5;
    println!("{}",x);
    x = 6;
    println!("{}",x);

    //Integers
    let small_number: u8 = 1;
    let big_number: u128 = 123456789012345678901234567890;
    println!("{}",small_number);
    println!("{}",big_number);

    //float
    let float_number: f64 = 123456789012345678901234567890.0;
    println!("{}",float_number);

    //boolean
    let boolean_number: bool = false;
    println!("{}",boolean_number);

    if boolean_number {
        println!("hello world");
    }else {
        println!("hello rust");
    }

    let b:bool;
    b = false;
    println!("{}",b);

    //char
    let c = 'a';
    let d = 'b';
    println!("{} {}",c,d);


    //loop .. range
    for i in 1..10 {
        println!("{}",i);
    }

    for i in 'a'..'z' {
        println!("{}",i);
    }

    //tuple
    let tup:(i32,f64,char) = (500,6.4,'a');
    let (x,y,z) = tup;
    println!("{} {} {}",x,y,z);

    let fiv_hundred = tup.0;
    println!("{}",fiv_hundred);

    let six_point_four = tup.1;
    println!("{}",six_point_four);
}
