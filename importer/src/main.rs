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
    if args.len() < 2 {
        return Err(format!("Falta parâmetro! Utilize: {} nome_do_arquivo_zip", args[0]));
    }

    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
       
        if (&*file.name()).ends_with('/') {
            continue;
        } else {
            match Config::new(&*file.name()) {
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
