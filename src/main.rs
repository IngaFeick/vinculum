fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Please provide an arabic number or a roman numeral.");
    if arg.chars().all(char::is_numeric) {
        let arabic = arg.parse::<u64>().unwrap();
        let result = vinculum::arabic2vinculum(arabic).unwrap();
        println!("{}", result);
    } else {
        let result = vinculum::vinculum2arabic(&arg);
        match result {
            Ok(value) => println!("{}", value),
            Err(error) => panic!("Problem: {:?}", error),
        };
    };
}
