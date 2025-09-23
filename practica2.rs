use std::io;

fn main() {
    println!("Ingrese un número:");

    let mut e_u = String::new(); 
    io::stdin()
        .read_line(&mut e_u)
        .expect("Fallo en leer la línea"); 

    let numero: i32 = e_u.trim().parse().expect("Por favor, introduce un número válido"); 

    println!("Has introducido: {}", numero);

    if numero % 2 == 0 {
        println!("El número {} es PAR", numero);
    } else {
        println!("El número {} es IMPAR", numero);
    }
}

