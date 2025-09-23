use std::io; 


fn main() {
let mut num = String::new(); 

println!("Ingresa un número: "); 
io::stdin().read_line(&mut num).unwrap();
 
let numero: i32 = num.trim().parse().unwrap();
 
if numero % 2 == 0 {
println!("Es un número par :) :) ");
 } 
else{
println!(" es un número par   :(:( "); 
} 
} 
