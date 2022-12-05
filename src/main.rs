fn main() {
    // Print Hello World
    println!("Hello, world!");

    let my_inmutax = 10;
    // Print a number
    println!("The value of x is: {}", my_inmutax);

    let mut my_mutex_var = 10;
    println!("The value of my_mytex_var: {}", my_mutex_var);
    my_mutex_var = 20;
    println!("The value of my_mytex_var new value: {}", my_mutex_var);

    const PI: f32 = 3.1416;
    println!("The value of PI is: {}", PI);

    let my_inmutax = "my new value";
    println!("The value of x is: {}", my_inmutax);
}
