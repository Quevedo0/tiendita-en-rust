use std::io;
use std::str::FromStr;

fn main(){

println!("Ingresar numero");
let mut numero=String::new();
io::stdin().read_line(& mut numero).expect("Error de entrada");

let result : u8 =
u8::from_str(&numero.trim()).unwrap();

if result%2==0{
	println!("El numero es par");
}
else{
	println!("El numero es impar");
}
}
