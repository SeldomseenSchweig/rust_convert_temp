use std::io;

fn main() {
    
     
    

    let run = true;
    


    

    while run == true {

        println!("Would you like to convert celsius to fahrenheit or fahrenheit to celsius?");


        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line.");

        if answer.trim() == "no" || answer.trim() == "no" {
            println!("Goodbye!");
            break;
        }



        println!("Well then, which would like to convert?");

        let mut convert = String::new();

    io::stdin()
        .read_line(&mut convert)
        .expect("Failed to read line.");


     if convert.trim() ==  "celsius" || convert.trim() == "Celsius" {

        println!(" Input the temperature and it will be converted to fahrenheit");

        let mut celsius = String::new();

        io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line.");

        let celsius: f32 = celsius.trim().parse().unwrap();

        

            println!("The temperature in fahrenheit is: {} ", convert_to_fah(celsius)); // );


        }

        else if convert.trim() == "fahrenheit" || convert.trim() == "Fahrenheit"  {

        println!(" Input the temperature and it will be converted to Celsius");

        let mut fehrenheit = String::new();

        io::stdin()
        .read_line(&mut fehrenheit)
        .expect("Failed to read line.");

        let fehrenheit: f32 = fehrenheit.trim().parse().unwrap();

        

            println!("The temperature in fahrenheit is: {} ", convert_to_cel(fehrenheit));

          
    }

    

    }




}

 fn convert_to_fah(x: f32) -> f32{

    x * 9.0/5.0 + 32.0
    
 }

fn convert_to_cel(x: f32) -> f32{

    (x - 32.0) * 5.0/9.0 

}