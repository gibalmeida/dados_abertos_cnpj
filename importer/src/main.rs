use std::path::PathBuf;
use std::{fs, io};

use importer::cli::Cli;
use importer::config::Config;
use importer::import::Import;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    std::process::exit(match real_main(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Erro: {:?}", err);
            1
        }
    })
}

fn real_main(args: Cli) -> Result<(), String> {

    if args.folder {
        return import_from_dir(&args)
    }

    return import_from_file(&args, &args.path_to_import);
}

fn import_from_dir(args: &Cli) -> Result<(), String> {
    
    let path_to_import = &args.path_to_import;
    println!("Importando arquivos a partir do diretório: {}", path_to_import.display());

    if fs::Metadata::is_file(&path_to_import.metadata().unwrap()) {
        return Err(String::from("Você informou um arquivo como argumento! Com o flag -d como argumento você deve informar um caminho de diretório onde estão os arquivos para serem importados"));
    }

    let entries_result = fs::read_dir(path_to_import);

    let mut entries = match entries_result {
        Ok(entries) => entries
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().unwrap(),
        Err(error) => return Err(format!("Ocorreu um erro ao ler o diretório {}: {:?} ", path_to_import.display(), error))   
    };

    entries.sort();

    for entry in entries {
        if let Some(ext) = entry.extension() {
            if ext == "zip" {
                    if fs::Metadata::is_file(&entry.metadata().unwrap()) {
                        println!("Importando o arquivo {} ...", entry.display());
                        if let Err(error) = import_from_file(&args, &entry) {
                            return Err(format!("Ocorreu um erro ao processar o arquivo {}: {:?}", entry.display(), error));
                        }
                        println!("=========");
                    }
            }
        }
    }

    Ok(())      
}

fn import_from_file(args: &Cli, file_to_import: &PathBuf) -> Result<(), String> {

    if fs::Metadata::is_dir(&file_to_import.metadata().unwrap()) {
         return Err(String::from("Você informou um diretório como argumento! Você deve informar um arquivo para ser importado."));
    }

    let file = match fs::File::open(file_to_import) {
        Ok(f) => f,
        Err(error) => return Err(format!("Ocorreu um erro ao abrir o arquivo {}: {:?}", file_to_import.display(), error))
    };

    let mut archive = zip::ZipArchive::new(file).unwrap();
    
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
       
        if (&*file.name()).ends_with('/') {
            continue;
        } else {
            match Config::new(&*file.name(), &args) {
                Ok(config) => {
                    let mut import = Import::new(&config);
        
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
