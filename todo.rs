use std::io::{self, Write};
use std::process::Command;

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()

}

struct Tarefa{
    nome: String
}

fn add(nome: String, lista: &mut Vec<Tarefa> ){
    lista.push(Tarefa{nome});
}

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}


fn main(){
    let mut lista :Vec<Tarefa> = vec![];
    loop{
        limpar_tela();
        if !lista.is_empty(){
            println!("===== Lista de Tarefas =======");
            for (id, task) in lista.iter().enumerate() {
                println!("{} - {}", id, task.nome);
            }
        }else{
            println!("A lista ainda está vazia!");
        }
        let nome = input("Digite a tarefa que deseja adicionar: ");
        if nome.as_str() == "00" || nome.is_empty(){
            loop {
                println!("==== Menu de opções ====");
                println!("A) Adicionar tarefa: ");
                println!("B) Remover tarefa:");
                let opcao = input("Digite a opção desejada: ");
                if opcao == "A".to_owned(){
                    break;
                }else if opcao == "B".to_owned() {
                    let indice = input("Digite o numero da tarefa que deseja remover: ").parse::<usize>().unwrap();
                    lista.remove(indice);
                    break;
                }else{
                    println!("Opção invalida, escolha A ou B");
                }
            }
        }else{
            add(nome, &mut lista);
        }
    }

}