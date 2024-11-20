fn main() {
    println!("Hello, world!");
    
    another_function(1977, 'y');
}

fn another_function(x: u32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}
