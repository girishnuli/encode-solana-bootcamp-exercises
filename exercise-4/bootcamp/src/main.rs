fn fizz_buzz() {

    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        if i % 3 == 0 {
            println!("{} - fizz", i);
        } 
        
        if i % 5 == 0 {
            println!("{} - buzz", i);
        } 
        
        if i % 3 == 0 && i % 5 == 0 {
            fizz_buzz_count += 1;
            println!("{} - fizz buzz", i);
        }
    }

    println!("fizz buzz occurred {} times.", fizz_buzz_count);
}

fn main() {
    println!("Welcome to Fizz Buzz");
    fizz_buzz();
}
