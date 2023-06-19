use std::path::Path;
use std::fs;

mod ignore;

fn main() {
    println!("Iniciando teste da funcionalidade \"Ignore List\"\n");
    
    ignore::start_ignore();
    aux_teste_ignore(false);
    ignore::end_ignore();
    aux_teste_ignore(true);
}

// Função auxiliar para verificar o Ignore List
fn aux_teste_ignore(is_ended: bool){
    if !is_ended {
        println!("Verificando se o arquivo foi movido com sucesso");
        for line in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
            if !Path::new(line).exists() {
                println!("\x1b[32mTeste Passou!\x1b[0m Com o caminho: \x1b[33m{line}\x1b[0m");
            }
            else {
                println!("\x1b[91mTeste Falhou!\x1b[0m Com o caminho: \x1b[33m{line}\x1b[0m");
            }
        }
    }
    else {
        println!("\nVerificando se o arquivo voltou para o seu local de origem");
        for line in fs::read_to_string("config/ignore_list.conf").unwrap().lines() {
            if Path::new(line).exists() {
                println!("\x1b[32mTeste Passou!\x1b[0m Com o caminho: \x1b[33m{line}\x1b[0m");
            }
            else {
                println!("\x1b[91mTeste Falhou!\x1b[0m Com o caminho: \x1b[33m{line}\x1b[0m");
            }
        }
    }
}