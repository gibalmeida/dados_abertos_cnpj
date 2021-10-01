use std::str::FromStr;
use crate::types::TipoDeArquivo;

pub struct Config {
    tipo_de_arquivo: TipoDeArquivo,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Parâmetros insuficientes! Por favor, execute o comando com um tipo de arquivo como parâmetro. Os tipos válidos de arquivos são: empresas, estabelecimentos, cnaes, naturezas_juridicas, paises, municipios, qualificacoes_de_socios, motivos_de_situacoes_cadastrais e ...")
        }

        let tipo_de_arquivo = match TipoDeArquivo::from_str(&args[1]) {
            Ok(v) => v,
            Err(_e) => return Err("Tipo de arquivo inválido")
        };

        Ok(Config {tipo_de_arquivo})
    }

    pub fn tipo_de_arquivo(&self) -> &TipoDeArquivo {
        &self.tipo_de_arquivo
    }
}
