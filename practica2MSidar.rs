use std::io;

	fn main() {
	println!("Por favor, ingresa un número:");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Error al leer la línea");

	let num: i8 = input.trim().parse().expect("Por favor, ingresa un número válido");

	if num % 2 == 0 {
	println!("El número {} es par.", num);
	} else {	
	println!("El número {} es impar.", num);
	}
}



