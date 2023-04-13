use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game");
    let secret_number=rand::thread_rng().gen_range(1..101);
   // println!("Secret Number is {}", secret_number);
    loop{
    println!("Please input your guess");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess is: {}",guess);
    let guess:u32=guess.trim().parse().expect("Type an integer");
    //println!("{}",guess+15);
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too small"),
        Ordering::Greater =>println!("Too big"),
        Ordering::Equal =>{
            println!("You Win");
            break;
    },
}
}
}
