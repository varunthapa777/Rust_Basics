fn main() {
    //1. temprature conversion
    let fahrenheit = 100.0;

    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);

    let celsius = 36.0;

    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);

    //2. fibonacci
    fibonacci_series(2);
    print_christmas_coral();
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fibonacci_series(n: u32) {
    let mut t1 = 0;
    let mut t2 = 1;

    print!("{} {} ", t1, t2);
    for _i in 2..n {
        let next_term = t1 + t2;
        print!("{next_term} ");
        t1 = t2;
        t2 = next_term
    }
    println!();
}

fn print_christmas_coral() {
    let lines = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
    ];
    let ordinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas,", ordinal_numbers[i]);
        println!("my true love gave to me");

        for j in lines.len() - i..lines.len() {
            println!("{}", lines[j]);
        }
        println!("And a partridge in a pear tree!");
        println!();
    }
}
