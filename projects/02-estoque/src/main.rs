use std::io;
use std::collections::HashMap;
#[derive(Debug)]
struct Produto {
    nome: String,
    quantidade: u32,
    preco: f32,
}
fn main() {
    let mut estoque: HashMap<String, Produto> = HashMap::new();

    loop {
        println!("=== Estoque ===");
        println!("1- Adicionar produto");
        println!("2- Remover produto");
        println!("3- Listar produtos");
        println!("4- Sair");

        println!("Escolha uma opção: ");
        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler opção");

        match opcao.trim() {
            "1" => adicionar_produto(&mut estoque),
            "2" => remover_produto(&mut estoque),
            "3" => listar_produtos(&estoque),
            "4" => break,
            _ => println!("Opção invalida"),
        }
    }
}
fn adicionar_produto(estoque: &mut HashMap<String, Produto>) {
    println!("Digite o nome do produto: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler nome do produto");
    let nome = nome.trim().to_string();

    println!("Digite a quantidade de produto: ");
    let mut quantidade = String::new();
    io::stdin().read_line(&mut quantidade).expect("Erro ao ler quantidade de produto");
    let quantidade: u32 = match quantidade.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um número válido!");
            0
        }
    };

    println!("Digite o preço do produto: ");
    let mut preco = String::new();
    io::stdin().read_line(&mut preco).expect("Erro ao ler o preço");
    let preco: f32 = match preco.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um valor válido!");
            0.0
        }
    };

    let produto = Produto {
        nome: nome.clone(),
        quantidade,
        preco,
    };

    estoque.insert(nome, produto);

    println!("Produto adicionado com sucesso!");
    
}

fn remover_produto(estoque: &mut HashMap<String, Produto>) {
    println!("Digite o nome do produto que deseja remover: ");
    let mut nome: String = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler nome do produto");
    let nome: String = nome.trim().to_string();

    if estoque.remove(&nome).is_some() {
        println!("Produto removido com sucesso!");
    } else {
        println!("Produto não encontrado");
    }
}

fn listar_produtos(estoque: &HashMap<String, Produto>) {
    for produto in estoque.values() {
        println!("Produto: {} | Quantidade: {} | Preço: R${}", produto.nome, produto.quantidade, produto.preco);
    }
}