use std::fs;

use importer::config::Config;
use importer::import::Import;

fn main() {
    std::process::exit(match real_main() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}

fn real_main() -> Result<(), String> {

    let args: Vec<String> = std::env::args().collect();
    let filename;
    let mut records_limit = 1000usize;

    match args.len() {
        2 => {
            filename = &args[1];
        },
        4 => {
            let cmd = &args[1];
            let arg = &args[2];
            filename = &args[3];

            match &cmd[..] {
                "-n" => {
                    records_limit = match arg.parse() {
                        Ok(n) => {
                            n
                        },
                        Err(_e) => {
                            help(&args[0]);
                            return Err(format!("Valor inválido para -n: {}", arg));
                        }
                    };
                },
                _ => {
                    help(&args[0]);
                    return Err(format!("Opção inválida: {}", cmd));
                }

            }
        },
        _ => {
            help(args[0].as_ref());
            return Err(String::from("Nenhum parâmetro informado ou informados incorretamente."));
        }
    }

    let fname = std::path::Path::new(filename);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
       
        if (&*file.name()).ends_with('/') {
            continue;
        } else {
            match Config::new(&*file.name(), records_limit) {
                Ok(config) => {
                    let import = Import::new(config);
        
                    import.run(file)?;
                },
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        }
    }

    Ok(())
}

fn help(bin_path: &str) {
    println!("
Importador de arquivos CSV da Receita Federal para um Banco de Dados MySQL

Uso: {} [opções] <caminho_do_arquivo_csv_zip>

O caminho_do_arquivo_csv_zip deve ser obrigatóriamente informado, e especifica o caminho completo (pasta + nome do arquivo) para o arquivo CSV no formato .zip que será processado pelo importador.

Opções:
    -n Número       informa a quantidade de registros a ser inseridos/alterados de uma única vez na tabela empresa, 
                    estabelecimentos, socios ou simples. Esta quantidade pode aumentar a performance de inclusão de
                    registros nas tabelas, porém, aumentar de mais pode reduzir a performance, bem como ocasionar
                    erros durante a importação, dependendo da configuração do banco de dados. Por padrão este valor
                    é igual a 1000.
                    
", bin_path);
}