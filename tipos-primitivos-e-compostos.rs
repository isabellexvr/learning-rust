fn main() {
    let x: usize = 10; // O tipo depende da arquitetura

    if cfg!(target_pointer_width = "64") {
        println!("Estamos em um sistema de 64 bits.");
    } else {
        println!("Estamos em um sistema de 32 bits.");
    }

    println!("O valor de x é: {}", x);

    // tipos primitivos: inteiros, floats, booleanos, caracteres

    // ints: signed e unsigned
    // signed: i8, i16, i32, i64, i128, isize -> de 8 até 128 bits -> isize depende da arquitetura
    // unsigned: u8, u16, u32, u64, u128, usize -> de 8 até 128 bits -> usize depende da arquitetura

    // floats: f32, f64

    // booleanos: bool

    let int1: i32 = -10;
    let int2: u8 = 255;
    println!("inteiros: {} {}", int1, int2);

    let float1: f32 = 1.0;
    let float2: f64 = 1.0;
    println!("oi\n");
    println!("floats: {} {}", float1, float2);

    let bool1: bool = true;
    println!("booleano: {}", bool1);

    let char1: char = 'a';
    println!("caractere: {}", char1);

    // compound data types: tuples, arrays, slices and strings (+ slice string)

    // arrays: 
    let numbers = [1, 2, 3, 4, 5];
    let numbers1: [i32; 5] = [1, 2, 3, 4, 5]; // define o tipo e o tamanho do array
    // ambos, acima, são iguais, mas o tipo e o tamanho no primeiro são deduzidos pelo compilador

    // maneiras de printar:
    println!("numbers: {:?}", numbers);
    println!("numbers: {:#?}", numbers); // print mais detalhado (quebra de linha a cada elemento)
    //println!("numbers1: {:?}", numbers1);

    //let mixed = [1, 2, true]; // invoca um erro, porque toda array deve conter o mesmo tipo de dados dentro
    //println!("mixed: {:?}", mixed);

    let string_arr: [&str; 2] = ["hello", "world"]; // str = string slice
    println!("string_arr: {:?}", string_arr);

    // tuples:
    let tuple:(i8, &str, bool, f32) = (1, "banana", true, 3.14);
    println!("tuple: {:?}", tuple);
    let supermixtuple = (1, "banana", true, 3.14, [1, 2, 3], ("tuple", "inside", "tuple"));
    println!("supermixtuple: {:?}", supermixtuple);
    // converter string slice para string:
    let string = tuple.1.to_string();
    println!("string: {}", string);

    // slices: reference a contiguous (lado a lado na memoria) sequence of elements in a collection rather than the whole collection

    let name = "John Doe"; // isso é um string slice -> imutável, como array
    let mut name_string = String::from("John Doe"); // isso é um string -> mutável
    name_string.push('!'); // invoca um erro, porque string slices são imutáveis
    println!("name_string: {}", name_string);

}