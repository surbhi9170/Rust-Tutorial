fn main() {
    println!("Hello, world!");

    //basics of variables

    // let mut x= 4;
    // println!("value of X is {}",x);
    // x= 5;
    // println!("value of X is {}",x);

    let x= 4;
    println!("value of X is {}",x);

    {
        let x= 2;
        println!("value of X is {}",x);
    }

    {
        let x= x-2;
        println!("value of X is {}",x);
    }
    let x= x+1;
    println!("value of X is {}",x);

    let x= "hello";
    println!("value of X is {}",x);

    //constants basics 

    //defining a data type is a must 
    const SECONDS_TO_MINUTES : u32 = 60;
    println!("{}",SECONDS_TO_MINUTES);


    //you cannot define same constant again this line will give ann error
    // const SECONDS_TO_MINUTES : u32 = 30;
    // println!("{}",SECONDS_TO_MINUTES);
}
// cargo run (to compile and run)
// cargo build (to compile)
// cargo check (to check for errors)
// goto source folder and type rustfmt main.rs to format the file 

