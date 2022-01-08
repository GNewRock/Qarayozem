use std::io;

fn main() {
    println!("guess the number!");

println!("please enter a guess!");

let mut gues=String::new();
io::stdin().read_line(&mut guess)
.expert("could not read the line.");
println!("your try:{}", guess);
}
