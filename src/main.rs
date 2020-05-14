use std::io;
fn main() {
    println!("input any number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input1:i32 =input.trim().parse().unwrap();

    if input1 % 3 == 0 && input1 % 5 != 0 && input1 % 7 !=0 {
       println!("Main in ko Rulaunga");
    }
    else if input1 % 5 == 0 {
        if input1 % 3 == 0 {
            println!("Main in ko Rulaunga");
        }
        println!("Mujhe Kion Nikala");  
    }
    else if input1 % 7 == 0 {
        println!("Barish hoti he to pani aata he");
    }

let x = prime(input1);
   if x == true {
    println!("Tabdeeli agai he");
    }

if input1 % 3 != 0 && input1 % 5 != 0  && input1 % 7 !=0 && x == false {
println!("The num is {}", input1);
}
   

}

       


fn prime(n: i32) -> bool { 
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true 
}