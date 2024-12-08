use std::io::{self, Write};
use std::process::Command;

struct Tarefa{
    nome: String
}

fn input(texto:&str) -> String{ //Entrada de texto
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()

}

fn exibir_lista(lista: &Vec<Tarefa>){ // Exbie a lista
    if !lista.is_empty(){
        println!("===== Lista de Tarefas =======");
        for (id, task) in lista.iter().enumerate() {
            println!("| {} - {}", id+1, task.nome);
        }
    }else{
        println!("A lista está vazia!");
    }
    println!("==============================")
    
}

fn add(nome: String, lista: &mut Vec<Tarefa> ){ // Adiciona novas tarefas a lista
    lista.push(Tarefa{nome});
}

fn del(lista: &mut Vec<Tarefa>){ // Remove uma tarefa em um indice especifico
    loop{
        let indice = input("Digite o numero da tarefa que deseja remover: ");
        match indice.parse::<usize>() {
            Ok(indice) => {
                if indice-1 <= lista.len(){
                    lista.remove(indice-1);
                    break;
                }else{
                    println!("Digite um numero valido!");
                }
            },

            Err(_) => println!("Valor invalido!")
        }

        
    }
}

fn limpar_tela() {  // Limpa a tela anterior no inicio de cada loop
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
        exibir_lista(&lista);
        println!();

        let nome = input("Digite a tarefa que deseja adicionar ou <ENTER> para remover: ");
        if !nome.is_empty(){   // Verifica se o usuario digitou uma tarefa. 
            add(nome, &mut lista);
        }else{
            if !lista.is_empty(){  // Verifica se existe Items para serem removidos
                del(&mut lista);
                continue;
            }
        }
    }

}