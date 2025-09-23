use std::io;

fn main(){

println!("Ingrese un numero:");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Error al leer");
	
	let numero: i8= input.trim().parse().expect("Debe ingresar un numero valido");
	
	if numero % 2 == 0{
	println!("El numero {} es PAR", numero);
}else{
	println!("El numero {} es IMPAR", numero);
}
	
}
