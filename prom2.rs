use std::io;
use std::str::FromStr;

fn main(){

println!("Dame un numero entero:");
let mut numero = String::new();
io::stdin().read_line(&mut numero).ok();

let edad : u8 =
u8::from_str(&numero.trim()).unwrap();
let frase = if edad % 2 == 0 {

"Tu numero es par"
}
else{
"Tu numero es impar"
};

println!("{}",frase);
}
