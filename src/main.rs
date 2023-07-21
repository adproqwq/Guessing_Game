use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Please guess a number, and enter it to me.");
    let unknown_number = rand::thread_rng().gen_range(1..=100);
    //println!("<test>The unknown number is {unknown_number}");
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("I can't read the number");
        let num:u32 = num.trim().parse().expect("It's not a number! Please enter a number to me!");
        match num.cmp(&unknown_number) {
            Ordering::Less => println!("It's too small! Guess again!"),
            Ordering::Equal => {
                println!("You get it!");
                break;
            },
            Ordering::Greater => println!("It's too big! Guess again!"),
        }
    }
}
