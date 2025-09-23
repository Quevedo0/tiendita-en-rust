//Arredondo Tapia 
fn main(){
let mut numero = String::new();
println!("Ingrese un numero: ");
std::io::stdin().read_line(&mut numero).expect("Error al leer el numero");
let numero: i32 = numero.trim().parse().expect("Error al convertir el numero");
if numero % 2 == 0 {
    println!("El numero {} es par", numero);
} else {
    println!("El numero {} es impar", numero);


}
}