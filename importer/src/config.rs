use std::str::FromStr;
use crate::tipo_de_arquivo::TipoDeArquivo;
use crate::cli::Cli;

pub struct Config<'a> {
    tipo_de_arquivo: TipoDeArquivo,
    args: &'a Cli,
}

impl<'a> Config<'a> {
    pub fn new(filename: &str, args: &'a Cli) -> Result<Config<'a>, &'static str> {

        match TipoDeArquivo::from_str(&filename) {
            Ok(v) => Ok(Config{tipo_de_arquivo: v, args}),
            Err(_) => Err("Tipo de arquivo inválido!")
        }
    }

    pub fn tipo_de_arquivo(&self) -> &TipoDeArquivo {
        &self.tipo_de_arquivo
    }
    pub fn rows_per_insert(&self) -> usize {
        self.args.rows_per_insert
    }
    pub fn verbose(&self) -> bool {
        self.args.verbose
    }
}

