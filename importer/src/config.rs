use std::str::FromStr;
use crate::tipo_de_arquivo::TipoDeArquivo;
use crate::cli::Cli;

pub struct Config<'a> {
    tipo_de_arquivo: TipoDeArquivo,
    file_part_number: u8,
    args: &'a Cli,
}

impl<'a> Config<'a> {
    pub fn new(filename: &str, args: &'a Cli) -> Result<Config<'a>, &'static str> {

        let mut file_part_number = 0;
        if let Some(s) = filename.find(".K03200Y") {
            file_part_number = filename[s+8..s+9].parse().unwrap();
        }

        match TipoDeArquivo::from_str(&filename) {
            Ok(v) => Ok(Config{tipo_de_arquivo: v, file_part_number, args}),
            Err(_) => Err("Tipo de arquivo inválido!")
        }

    }

    pub fn tipo_de_arquivo(&self) -> &TipoDeArquivo {
        &self.tipo_de_arquivo
    }

    pub fn is_first_file_number(&self) -> bool {
        self.file_part_number == 0
    }

    pub fn is_last_file_number(&self) -> bool {
        self.file_part_number == match self.tipo_de_arquivo() {
            TipoDeArquivo::Empresas => 9,
            TipoDeArquivo::Estabelecimentos => 9,
            TipoDeArquivo::Socios => 9,
            _ => 0
        }
    }


    pub fn rows_per_insert(&self) -> usize {
        self.args.rows_per_insert
    }

    pub fn verbose(&self) -> bool {
        self.args.verbose
    }

    pub fn empty(&self) -> bool {
        self.args.empty
    }

    pub fn truncate_table(&self) -> bool {
        self.args.truncate_table
    } 

    pub fn drop_indexes(&self) -> bool {
        self.args.drop_indexes
    }
}

