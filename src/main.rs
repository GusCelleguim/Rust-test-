fn bhaskara(a:i32, b:i32,c:i32){
    let delta = b.pow(2)-4*a*c;
    let (mut x1, mut x2): (i32, i32) = (0, 0);
    if delta < 0  {
        println!("Impossivel")
    }else {
        x1 = ((-1*b) + (delta as f64).sqrt() as i32) / 2*a;
        x2 = ((-1*b) - (delta as f64).sqrt() as i32) / 2*a;
        println!("as raizes são: {} {} ", x1,x2)
    }}

fn main () {
bhaskara(1,10,10)
}



// u8 ------------> natural de 8bits ---> que vai de 5 a 12000

// STRING-------> string que pode ser mudada 
// str---------> String que acaba nao mudando  


/*
struct Pessoa{
    nome: String,
    idade: u8,
    escola: String 

}


fn main() {
println!("Hello, world!");

let gustavo = Pessoa {
    idade: 18,
    escola: String::from("Colegio Absoluto"),
    nome: String::from("Gustavo Celleguim")
};

let idade_celleguim= gustavo.idade;


println!("{}",gustavo.nome);
println!("{}",idade_celleguim);
println!("{}",gustavo.escola);
println!("a pessoa {} tem {} estuda na {}",gustavo.nome,gustavo.idade,gustavo.escola);

 */