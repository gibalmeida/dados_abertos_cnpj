use std::env;
use std::process;

use dados_abertos_cnpj::import::Import;
use dados_abertos_cnpj::config::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ocorreu um erro ao analisar os parâmetros do comando: {}", err);
        process::exit(1);
    });

    let import = Import::new(config);

    import.run();

}

