//Emilio Rafael Beltran Barraza
fn main(){
let mut numero = String::new();


println!("ingrese el numero:");
std::io::stdin().read_line(&mut numero).expect("error al leer el numero");
let numero: i32 = numero.trim().parse().expect("error al convertir el numero");
if numero %2 == 0{
	println!("el numero {} es par", numero);
}else{
	println!("el numero {} es impar", numero);
}


}
