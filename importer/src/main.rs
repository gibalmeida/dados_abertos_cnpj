use std::fs;

use importer::cli::Cli;
use importer::config::Config;
use importer::import::Import;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    std::process::exit(match real_main(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}

fn real_main(args: Cli) -> Result<(), String> {

    let file = fs::File::open(&args.file_to_import).unwrap();

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
