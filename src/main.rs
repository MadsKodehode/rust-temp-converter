fn main() {
    loop {
        println!("What type do you want to convert?");
        println!("1> °C");
        println!("2> °F");
        let mut input = String::new();

        //Construct new handle to std input
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read convertion type.");

        //Return string as number
        let convertion_type = match input.trim() {
            "1" => 1,
            "2" => 2,
            _e => {
                println!("Please select either 1 or 2");
                continue; //Allow user to input again
            }
        };

        println!("Enter the temperature: ");
        let mut temperature = String::new();

        std::io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line: temperature");

        //Convert the temprature to number
        let temp: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature");
                continue;
            }
        };

        if convertion_type == 1 {
            let fahrenheit = (temp * 1.8) + 32.;
            print!("{temp}°C is {fahrenheit}°F\n");
        } else {
            let celsius = (temp - 32.) / 1.8;
            print!("{temp}°F is {celsius}°C\n");
        }
    }
}
