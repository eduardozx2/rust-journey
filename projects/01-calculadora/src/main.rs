use std::io;

fn main() {
    loop {
        println!("=== Calculadora ===");
        println!("Digite o primeiro número: "); // pede o primeiro número para o usuário
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Erro ao ler número");
        let num1: f64 = match num1.trim().parse() { // tenta converter a entrada para um número de ponto flutuante
            Ok(n) => n,
            Err(_) => {
                println!("Digite um número válido");
                continue;
            }
        };
        println!("Digite o segundo número: "); // pede o segundo número para o usuário
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Erro ao ler número");
        let num2: f64 = match num2.trim().parse() { // tenta converter a entrada para um número de ponto flutuante
            Ok(n) => n,
            Err(_) => {
                println!("Digite um número válido");
                continue;
            }
        };
        println!("1- Somar");
        println!("2- Subtrair");
        println!("3- Multiplicar");
        println!("4- Dividir");

        println!("Escolha uma operação: ");
        let mut operador = String::new();
        io::stdin()
            .read_line(&mut operador)
            .expect("Erro ao ler operação");

        let resultado = match operador.trim() { // verifica qual operação o usuário escolheu e chama a função correspondente
            "1" => somar(num1, num2),
            "2" => subtrair(num1, num2),
            "3" => multiplicar(num1, num2),
            "4" => dividir(num1, num2),
            _ => {
                println!("Opção invalida");
                0.0
            }
        };
        
        if num2!= 0.0 || operador.trim() != "4" { // verifica se a operação é divisão por zero antes de mostrar o resultado
            println!("Resultado: {:.2}", resultado);
        }

        println!("Calcular novamente? (s/n): ");
        let mut continuar = String::new(); 
        io::stdin()
            .read_line(&mut continuar)
            .expect("Erro ao ler opção");

        if continuar.trim() == "s" {
            continue;
        } else if continuar.trim() == "n" {
            break;
        }
    }
}
fn somar(a: f64, b: f64) -> f64 {
    a + b
}
fn subtrair(a: f64, b: f64) -> f64 {
    a - b
}
fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}
fn dividir(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Erro: divisão por zero!");
        0.0
    } else {
        a / b
    }
}
