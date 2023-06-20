use rpassword;

mod backup;

fn main() {
    println!("Iniciando teste de backup");

    let lista_senhas = vec!["senha123", "omd4urZt", "uw79trGh", "LPBdKM6y", "ozYwm6FK", "CsZvX4L6", "3LKfPatj",
    "FgRi9xwh", "RP7Ze2yg", "HCF9Lb3w"];

    for senha in lista_senhas {
        let mut pass_error = true;
        while pass_error {
            backup::init("teste_pasta", &senha.to_string());

            aux_teste_backup(&senha.to_string());
            pass_error = false;
        }
    }
}

// Função auxiliar para verificar se o backup foi feito corretamente
fn aux_teste_backup(senha: &str){
    use std::fs;
    use std::path::Path;

    match sevenz_rust::decompress_file_with_password("./teste_pasta/Backup teste_pasta.7z", "./teste_pasta_result",
        senha.into()) {
        Ok(_) => {
            let lista_arquivos = vec!("./teste_pasta_result/Backup teste_pasta/teste1.txt", 
            "./teste_pasta_result/Backup teste_pasta/teste2.txt",
            "./teste_pasta_result/Backup teste_pasta/teste3.txt",
            "./teste_pasta_result/Backup teste_pasta/teste pasta2/teste4.txt",
            "./teste_pasta_result/Backup teste_pasta/teste pasta2/teste pasta3/teste5.txt",
            "./teste_pasta_result/Backup teste_pasta/teste pasta2/teste pasta3/teste6.txt");

            for arquivo in lista_arquivos {
                if !Path::new(arquivo).exists() {
                    println!("\x1b[91mTeste Falhou!\x1b[0m");
                    return;
                }
            }

            fs::remove_dir_all("./teste_pasta_result").unwrap();
            fs::remove_file("./teste_pasta/Backup teste_pasta.7z").unwrap();
            println!("\x1b[32m\nTeste Passou!\x1b[0m");
        },
        Err(_) => {
            fs::remove_file(".teste_pasta.7z").unwrap();
            println!("\x1b[91mTeste Falhou!\x1b[0m");
        }
    }
}