use std::io::{self, Write};
use std::process::Command;

fn input(texto:&str) -> String{
    print!("{texto}");
    io::stdout().flush().unwrap();
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x

}

struct Tarefa{
    nome: String
}

fn add(nome: String, lista: &mut Vec<Tarefa> ){
    lista.push(Tarefa{nome});
}


fn main(){
    let mut lista :Vec<Tarefa> = vec![];
    loop{
        Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();


        if !lista.is_empty(){
            println!("===== Lista de Tarefas =======");
            for (id, task) in lista.iter().enumerate() {
                print!("{} - {}", id, task.nome);
            }
        }else{
            println!("A lista ainda está vazia!");
        }
        let nome = input("Digite a tarefa que deseja adicionar: ");
        add(nome, &mut lista);
    }

}