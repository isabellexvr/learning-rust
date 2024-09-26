//Macros permitem gerar e expandir código em tempo de compilação
//Ao contrário das funções (fn), macros "existem" apenas quando são "chamados". A execução do código ocorre em tempo de execução
//Quando você chama uma macro, o código correspondente é gerado e inserido no lugar da chamada da macro antes da compilação. Não há "execução" da macro em si; apenas a expansão do código ocorre.

fn quadrado (x: i32) -> i32{
    x*x
}

macro_rules! quadrado_macro{
    ($x:expr) => ($x * $x);
    //expr: expandir
    //=> o que é gerado pelo macro
}


fn main(){
    let n = 2;

    println!("O quadrado de {} é {}", n, quadrado_macro!(n));

}