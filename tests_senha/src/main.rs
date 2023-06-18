use std::fs;
use std::env;
use rpassword;

mod backup;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Para realizar o teste manual de senha. Use: ./tests_senha manual");
        println!("Para realizar o teste automático de senha. Use: ./tests_senha automatico");
        return;
    }

    if args[1] == "manual" { teste_manual(); }
    else if args[1] == "automatico" { teste_automatizado(); }
    else {
        println!("Para realizar o teste manual de senha. Use: ./tests_senha manual");
        println!("Para realizar o teste automático de senha. Use: ./tests_senha automatico");
    }
}

fn teste_manual(){
    println!("Iniciando teste manual de senha");

    let mut pass_error = true;
    while pass_error {
        let password = rpassword::prompt_password("Digite a senha: ").unwrap();

        if password == rpassword::prompt_password("Digite a senha novamente: ").unwrap() {
            backup::init("./teste_pasta", &password);

            pass_error = false;
        }
        else{
            println!("\x1b[31mSenha incorreta!\nTente novamente\n\x1b[0m");
        }
    }
}

fn teste_automatizado(){
    println!("Iniciando teste automatizado de senha");
    let lista_senhas = vec![
    "abcde12345", "CUyERJ9d", "UUmYK7he", "TyDeF74v", "fghi!@#678$%^", "jklm&*(90)", "nopq-_+rst", "uvw=;:xyz", "\"\'`~ß", "6aGgEGG6", "ZMfKJJD6", "sbMw3LyP", "jYGtcq94", "EUktfrk7", "uqHG6BkX", "NXwbu7zC", "mkiCN3Fw", "UepdB9Ln", "mqkctx6u", "2UQmrPvK",
    "ø®ŧ¥←↓↑→", "þÞáèª°īô", "ũçÇÆæ§Ðð", "đªŊảŁł«»", "©µ─×·÷ẹė", "JmrX5Cvy", "HA9xbhQS", "rNEGD5dT", "4TCrnQEj", "DzGenK5z", "EVTSNN35", "womRsnJ4", "cGCBJf9z", "7o2AsKuZ", "J6Gspbhm", "yW3DMVdq", "a2gAdrHA", "zckp3ZFM", "AGdQC2tg", "QTBs3YzX",
    "7weVPnoo", "CTWH4PCz", "KC9jJH7V", "BLy3Ypdv", "zQru2p9Y", "oC388GZM", "qir9HUGX", "cp2gFB7x", "Hn7zKfAG", "zuM48FXa", "qHeEur9o", "So9bn4CA", "CBZcWwW2", "g58t33uE", "PpcnQR2c", "55v7cnSG", "rv5drkqh", "aQCsM4f6", "qiBsP7JS", "7BHwkKKs",
    "xRe7vEnW", "GGQ5ncFo", "8mMfeob8", "zoy2rScf", "j3ZqVCFq", "SB2YpoSZ", "kPhUup6z", "HRd3gDCw", "CvwN5kQf", "7v8nZnYC", "ZLWrg2E9", "349ghh5M", "w8WD3eyo", "VVBB4fte", "tWV8YGsj", "8jgFbUGk", "9Exp9SFW", "D6ucoJqm", "zA4DeG9t", "CtXRwko3",
    "y9HssCsJ", "R8NhuLXF", "nK3Dxrqa", "XaQBHqU8", "w9zyPXJJ", "7mqg3vx4", "YcKn74Jg", "E2QbtuyG", "8jLjsgnr", "SEth6dNu", "ZP4uCB2J", "NEFWz8vS", "8JfxfFff", "FNur96AK", "HkeYSJ3f", "zD7tfTcX", "cMKqeh5c", "cdPnMXJ3", "qYyD566F", "8qDfxJxh",
    "AgjaN3nS", "rek66arv", "8cutRagW", "XSK9dzmz", "UoeCDQu3", "B5KheD8f", "Fjp2vuN7", "npvRz2BP", "jJfFQP2P", "ytL3m6K3", "vtBgLYu3", "KZJJ828o", "UTxCmJ22", "wVuP2J4y", "a4ajLDZR", "6UUSviAb", "t4VGkwon", "eS5KXjyF", "Y3HeJ74R", "bKZX8nXd"
    ];

    for senha_atual in lista_senhas {
        let mut pass_error = true;
        while pass_error {
            let password = senha_atual.to_string();

            if password == senha_atual.to_string() {
                backup::init("./teste_pasta", &password);

                aux_teste_senha(password.as_str());
                pass_error = false;
            }
            else{
                println!("\x1b[31mSenha incorreta!\nTente novamente\n\x1b[0m");
            }
        }
    }
}

// Função auxiliar para verificar se a senha foi devidamente aplicada
fn aux_teste_senha(senha: &str){
    match sevenz_rust::decompress_file_with_password("./teste_pasta.7z", "teste_pasta_result", 
        senha.into()) {
        Ok(_) => {
            fs::remove_dir_all("./teste_pasta_result").unwrap();
            fs::remove_file("./teste_pasta.7z").unwrap();

            println!("\x1b[32mTeste Passou!\x1b[0m Com senha: \x1b[33m{senha}\x1b[0m");
        },
        Err(_) => {
            fs::remove_file(".teste_pasta.7z").unwrap();

            println!("\x1b[91mTeste Falhou!\x1b[0m Com senha: \x1b[33m{senha}\x1b[0m");
        }
    }
}