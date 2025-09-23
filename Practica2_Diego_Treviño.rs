use std::io;

//DIEGO SANTOS TREVIÑO CAMACHO  

fn main() {

println!("Ingrese un numero para determinar si es par o impar: ");
let mut dato = String :: new();
io::stdin().read_line(&mut dato);


let numero: i8 = dato.trim().parse()
    .expect("No se pudo");


if numero % 2 == 0{
    println!("El numero es par.\n");
}else{
    println!("El numero es impar.\n");
}
}