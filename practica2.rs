use std::io;

fn main(){

let mut entrada = String::new();

println!("Escribe un numero");

io::stdin()
	.read_line(&mut entrada )
	.expect("Error al leer la entrada");

let valor: i16 = entrada.trim().parse().expect("Ingresa un numero valido");

if valor % 2 == 0{
println!("El numero {valor} es par.");
}
else {
println!("El numero {valor} es impar.");
}

}
