fn main() {
    greet();
    let name = "Varun";
    greet_with_name(name);

    let sum = add(5, 10);
    println!("Sum: {}", sum); 
   

}

// Function with no return type
fn greet() {
    println!("Hello, world!");
}


// Function with parameter
fn greet_with_name(name: &str){
    
    println!("Hello, {}", name);
}

// Function with return type
fn add(a: i32, b: i32) -> i32 {
    a + b
}