fn main() {

    // variable are immutable by default
    // let a = 5;

    // a = 6; // error: cannot assign twice to immutable variable `a`

    
    // Use mut keyword to make variable mutable
    let mut a = 5;
    
    println!("a = {a}");
    
    a = 6;
    
    println!("a = {a}");

    // shadowing
    let s = 20;

    let s = s + 1;


    {
        let s = s * 2;

        println!("Value of variable inside block: {s}");
    }

    println!("Value of variable outside block: {s}");
    
    // shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Value of spaces: {spaces}");
   
}
