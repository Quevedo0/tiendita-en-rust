use std::io;
fn main()
{
println!("Introduzca un numero para determinar si es par o impar:");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Fallo al leer linea");
let num:i32 = input.trim().parse().expect("");
if num%2==0 {
println!("El numero {} es par",num);}

else{
println!("El numero {} es impar",num);}

}
