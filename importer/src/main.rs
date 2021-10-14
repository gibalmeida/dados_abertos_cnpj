use std::env;
use std::process;

use importer::config::Config;
use importer::import::Import;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ocorreu um erro ao analisar os parâmetros do comando: {}", err);
        process::exit(1);
    });

    let import = Import::new(config);

    import.run();

}

