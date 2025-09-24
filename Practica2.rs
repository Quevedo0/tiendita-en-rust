use std::io;

fn main(){

println!("Ingrese un numero");
let mut Num = String::new();
io::stdin().read_line(&mut Num)
.expect("Error al leer");

let Num:i32=Num.trim().parse().expect("error al pasear el numero");

if Num%2==0
{
    println!("El numero es par");
}
else
{
        println!("El numero es impar");
}

}
