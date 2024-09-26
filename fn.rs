fn main(){
    hello_world(true);
    pessoa("Mark", 21, 1.7);
}

fn hello_world(boole: bool){
    if(boole){
        println!("Hello, world!");
    }else{
        println!("a");
    }
}

//sem retorno
fn pessoa(name: &str, idade: u32, altura: f32){
    println!("Nome: {}\nIdade: {}\nAltura: {}\n", name, idade, altura);
}

//com retorno
let x: i32 = {
    //code
}