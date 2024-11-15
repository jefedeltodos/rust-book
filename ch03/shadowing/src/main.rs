fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");
    
    shadow_to_chane_type();
}

fn shadow_to_chane_type() {
    let spaces = "    ";
    println!("The spaces look like '{spaces}'");

    let spaces = spaces.len();

    println!("The count of spaces is {spaces}");
}