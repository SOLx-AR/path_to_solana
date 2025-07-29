fn main() {
  //  let age:u8 = 9;
/*
    if age == 18 {
        println!("esta persona tiene 18");
    } else if age > 18 {
       println!("Esta persona tiene mas de 18") 
    }
     else {
        println!("es un niñe")
    }
 */
    //match
  /*  match age {
        18 => println!("esta persona tiene 18"),
        x if x < 18 => println!("es un niñe"),
        _ => println!("Esta persona tiene mas de 18"),
    }
*/

    let previusValue:f64 = 1.0;
    fn getInflation(country: &str) -> Option<f64>{
        //// llamado a una API
        /// si inflacion existe
        //None
        match country{
            "AR"=> Some(1.6),
            _ => None
        }
    }

    let inflation = getInflation("AR");

    //let currentValue = previusValue * if inflation == None { 1.0 } else {1.0 + (inflation.unwrap()/100.0)};

    let currentValue = match inflation {
        Some(x)=> 1.0 + x/100.0,
        None=> 1.0
    } * previusValue;


    println!("{currentValue}");

/*    if inflation == None {
        //no hago nada
    }else{
        println!("{}",inflation.unwrap());
    }
*/
/*
match inflacion {
    Some(x) => println!({x}),
    _ => {}
}
*//*
if let Some(x) = inflation{
    println!("{x}")
}
 */

    // en Rust no exiten los undefined, null
    // Option
    // Some(t)
    // None
/*
    fn getInflation(country: &str) -> Option<f64>{
        //// llamado a una API
        /// si inflacion existe
        //None
        Some(5)
    }

    let inflation = getInflation("AR").unwrap_or(0.0);

    println!("{inflation}");
*/
    // enum
   /* enum color {
        RGBA(u8,u8,u8,u8),
        EXA(String),
        blue,
        red,
    }

    enum Option {
        Some(T),
        None
    }

    enum Result {
        Ok(T),
        Err()
    }

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum direction {
        Up,
        Down,
        left,
        rigth
    }

    enum age {
        Some(u8),
        None
    }

    enum gender {
        Female,
        Male,
        Another(String),
        None
    }

    fn getAPIInfo() ->  Result<()> {
        //do some things
       //Ok()
       //Err()
    }
    */

/*let mut counter = 0;
    loop {
        println!("forever");
        counter= counter +1;
        if counter ==20 { break }
    }*/

    for x in 0..=5 {
        println!("{x}");
    }

    let mut counter = 0;
    while counter !=20 {
        println!("forever");
        counter= counter +1;
    }

    //map
    /*filter */
    //reduce
    //
}
