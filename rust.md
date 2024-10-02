# tipos primitivos

Inteiros:
````rust
let x: i32 = -10; // inteiro + e - de 32 bits 
let y: u64 = 100; // inteiro + de 64 bits
`````
Floats:
````Rust
let a: f32 = 3.14; // número de ponto flutuante de 32 bits 
let b: f64 = 2.718; // número de ponto flutuante de 64 bits
`````
Chars:
````Rust
let letter: char = 'R'; // caractere único
let emoji: char = '😊'; // caractere Unicode
`````
Tuplas:
````Rust
let tuple: (i32, f64, char) = (42, 6.28, 'x'); // tupla com três tipos diferentes
`````
Arrays:
````Rust
let arr: [i32; 3] = [1, 2, 3]; // array de inteiros com três elementos
`````

Vetor similar ao do C++:
````Rust
let mut v: Vec<i32> = Vec::new(); // Cria um vetor vazio de inteiros

v.push(10); // Adiciona um elemento ao vetor
v.pop(); // Remove e retorna o último valor do vetor, se existir
let first = v[0]; // Acessa o primeiro elemento (índice 0)
let len = v.len(); // Retorna o número de elementos no vetor
v.remove(1); // Remove o elemento na posição 1 (segundo elemento)
v.clear(); // Remove todos os elementos do vetor
let is_empty = v.is_empty(); // Retorna true se o vetor estiver vazio
v.resize(5, 0); // Redimensiona o vetor para ter 5 elementos, preenchendo novos com 0
let slice = &v[1..3]; // Retorna um slice dos elementos entre os índices 1 e 3 (não inclusivo)

// Percorrer os elementos:
for i in &v {
    println!("{}", i);
}

let mut v_initial = vec![1, 2, 3]; // Cria um vetor com elementos iniciais
`````

HashMap (key, value):
````Rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new(); // Cria um HashMap vazio

scores.insert(String::from("Blue"), 10); // Adiciona o par ("Blue", 10)
scores.insert(String::from("Red"), 50);  // Adiciona o par ("Red", 50)

// Acessar valores
// tendo-se uma string como key, procura-se o value
let team_name = String::from("Blue");
let score = scores.get(&team_name); // Retorna um `Option<&V>` que pode ser `Some(&valor)` ou `None`

if let Some(&score) = score {
    println!("O score de {} é {}", team_name, score);
}

// Remover elementos:
scores.remove(&String::from("Blue")); // Remove a entrada com a chave "Blue"

// Iterar:
for (key, value) in &scores { println!("{}: {}", key, value); }

// Atualizar valor:
scores.insert(String::from("Red"), 75); // Atualiza o valor associado à chave "Red"
`````

Match difere do Switch/Case
- correspondência de padrões mais complexos: usa condições, variáveis e tipos
- pode ser usado com enums, tuplas
- estritamente tipado, garantindo que os valores correspondam exatamente aos padrões
- O compilador exige que todos os casos sejam cobertos, ou que um caso padrão (`_`) seja fornecido. Isso ajuda a evitar erros em tempo de execução.

````Rust
fn condicao() {
	let número = 1; 
	match número {
		1 | 2 => println!("Um ou Dois"),
		3 => println!("Três"),
		_ => println!("Outro número"), 
	} 
}

fn variaveis() {
	let número = 1; 
	match número {
		1 | 2 => println!("Um ou Dois"),
		3 => println!("Três"),
		_ => println!("Outro número"), 
	} 
}

fn intervalo() {
	let nota = 85; // Nota do aluno 
	match nota { 0..=59 => println!("Reprovado"), // Notas de 0 a 59 
		60..=79 => println!("Aprovado com nota média"), // Notas de 60 a 79 
		80..=100 => println!("Aprovado com nota alta"), // Notas de 80 a 100 
		_ => println!("Nota inválida"), // Qualquer outra entrada 
	}
}

`````


O tipo Option:
- usado para representar um valor que pode ou não estar presente. É um tipo enum que pode ter dois variantes:
	- `Some(T)`: contém um valor do tipo `T`.
	- `None`: representa a ausência de valor.

````Rust
fn main() {
    let some_number: Option<i32> = Some(10); // Um valor presente
    let no_number: Option<i32> = None; // Ausência de valor

    match some_number {
        Some(value) => println!("O número é: {}", value),
        None => println!("Não há número."),
    }
}
`````

Structs:
 - agrupam campos inter-relacionados
 - similar às tuplas, mas cada entrada (variável) terá um nome e uma tipagem para ela
 - nome + tipo = campo

````Rust
fn main() {
    let user1 = User { 
	    active: true, 
	    username: String::from("someusername123"), 
	    email: String::from("someone@example.com"), 
	    sign_in_count: 1, 
	};
	let mut user1 = User { 
		active: true, 
		username: String::from("someusername123"), 
		email: String::from("someone@example.com"), 
		sign_in_count: 1, 
	}; 
		
	user1.email = String::from("anotheremail@example.com");
}
`````

Enums:
- enums fornecem uma maneira de dizer que um valor é um de um conjunto possível de valores

````Rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// tudo isso acima pode ser resumido em:

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// pode-se verificar os valores do enum com match:

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Received Quit message"),
        Message::Move { x, y } => println!("Moving to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to R: {}, G: {}, B: {}", r, g, b),
    }
}


`````

Namespaces
- são uma maneira de organizar e encapsular identificadores, como funções, tipos, constantes e módulos, para evitar conflitos de nome e tornar o código mais legível e organizado.

````Rust
mod my_module {
    pub fn my_function() {
        println!("Hello from my_function!");
    }
}

use my_module::my_function; // Traz `my_function` para o escopo

fn main() {
    my_function(); // Chama a função sem precisar do prefixo do módulo
}
`````

### Macros:
- permitem gerar código de forma programática. Elas ajudam a reduzir a repetição de código e podem ser usadas para simplificar tarefas comuns.

#### **Macros de Declaração (Declarative Macros)**: Usam a sintaxe `macro_rules!` e são utilizadas para gerar código com base em padrões. Elas são geralmente usadas para simplificar código repetitivo.

````Rust
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

fn main() {
    say_hello!(); // Chama a macro que imprime "Hello, World!"
}

`````

#### **Macros de Procedimento (Procedural Macros)**: Estas são um tipo mais avançado de macro que permite manipular o código de maneira mais complexa. Elas podem ser usadas para criar novos atributos ou derive (como `#[derive(Debug)]`), e são especialmente úteis para criar funcionalidades mais avançadas.

- Diferença entre função comum e macro declarativa:
````Rust
macro_rules! doubleMacroFunc {
    ($x:expr) => {
        $x * 2
    };
}

fn doubleFunc(x: i32) -> i32 { 
	x * 2 
}

fn main() {
    let resultMacro = doubleMacroFunc!(5); // Expande para `5 * 2`
    println!("Double: {}", resultMacro); // Imprime: Double: 10

	let resultFunc = double(5); // Chama a função 
	println!("Double: {}", resultFunc); // Imprime: Double: 10
}


`````

### A macro `vec!`

- ajuda sintaticamente a criar vetores!
- A macro `vec!` é definida na biblioteca padrão e simplifica a criação de vetores. Quando você usa `vec!`, a macro cria um novo vetor e o inicializa com os valores que você fornece.
- não precisa definir tipos

````Rust
fn main() {
    let mut scores = vec![100, 90, 80, 70];

    // Adicionando um novo score
    scores.push(85);
    println!("Scores: {:?}", scores); // Imprime: Scores: [100, 90, 80, 70, 85]

    // Iterando sobre os scores
    for score in &scores {
        println!("Score: {}", score);
    }
}
`````

