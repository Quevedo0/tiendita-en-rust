use std::io;

fn main(){
let mut input = String::new();

println!("Ingrese un numero");

io::stdin()
	.read_line(&mut input)
	.expect("error al leer la entrada");

let numero: i8 = input
.trim()
.parse()
.expect("POR FAVOR INTRUDUCE UN NUMERO VALIDO");


if numero % 2 == 0{
	println!("EL NUMERO ES PAR");
}else{
	println!("EL NUMERO ES IMPAR");
}

}
