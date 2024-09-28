fn main() {
    // let gender = "";
    // let age = 18;

    // if gender == "Male" {
    //     if age < 20 {
    //         println!("Yes Boy");
    //     } else {
    //         println!("Yes Sir");
    //     }
    // } else if gender == "Female" {
    //     if age < 20 {
    //         println!("Yes Girl");
    //     } else {
    //         println!("Yes Madam");
    //     }
    // } else {
    //     println!("👏 LGBTQ")
    // }

    // // Using if-else as expression
    // let condtion = true;
    // let num = if condtion { 4 } else { 5 };

    // println!("The value of num is: {}", num);

    // // loops and labels

    // let mut count = 0;
    // 'count_up: loop {
    //     let mut remaining = 10;

    //     println!("Count {}", count);
    //     loop {
    //         println!("Remaing {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'count_up;
    //         }

    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // // conditional loop using while

    // let mut n = 5;
    // while n != 0 {
    //     println!("{}!", "I Love Rust");
    //     n -= 1;
    // }

    // loop through a collection using for

    let a = [10, 20, 30, 40, 50];

    for el in a{
        println!("The value is: {}", el);
    }

    // make countdown using for loop

    for number in (1..=4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
