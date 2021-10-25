use std::str::FromStr;
use crate::tipo_de_arquivo::TipoDeArquivo;

pub struct Config {
    tipo_de_arquivo: TipoDeArquivo,
    records_limit: usize,
}

impl Config {
    pub fn new(filename: &str, records_limit: usize) -> Result<Config, &'static str> {

        match TipoDeArquivo::from_str(&filename) {
            Ok(v) => Ok(Config{tipo_de_arquivo: v, records_limit}),
            Err(_) => Err("Tipo de arquivo inválido!")
        }
    }

    pub fn tipo_de_arquivo(&self) -> &TipoDeArquivo {
        &self.tipo_de_arquivo
    }
    pub fn records_limit(&self) -> usize {
        self.records_limit
    }
}

