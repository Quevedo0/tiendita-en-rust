use std::io;

fn main() {
    
    println!("Ingresa n: ");
    let mut entrada = String::new();    
    io::stdin().read_line(&mut entrada)
        .expect("No se pudo leer");
        
    let numero: i8 = entrada.trim().parse()
        .expect("Ingrese un numero valido");
        
    if numero % 2 == 0 {
        println!("Tu numero es par");
        }
    else {
        println!("Tu numero es impar");
    }
} 
