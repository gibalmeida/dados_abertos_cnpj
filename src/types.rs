use std::str::FromStr;

pub enum TipoDeArquivo {
    Empresas,
    Estabelecimentos,
    CNAES,
    NaturezasJuridicas,
    QualificacoesDeSocios,
    Paises,
    Municipios
}

impl FromStr for TipoDeArquivo {

    type Err = ();

    fn from_str(input: &str) -> Result<TipoDeArquivo, Self::Err> {
        match input {
            "empresas"  => Ok(TipoDeArquivo::Empresas),
            "estabelecimentos"  => Ok(TipoDeArquivo::Estabelecimentos),
            "cnaes"  => Ok(TipoDeArquivo::CNAES),
            "naturezas_juridicas" => Ok(TipoDeArquivo::NaturezasJuridicas),
            "qualificacoes_de_socios" => Ok(TipoDeArquivo::QualificacoesDeSocios),
            "paises" => Ok(TipoDeArquivo::Paises),
            "municipios" => Ok(TipoDeArquivo::Municipios),
            _      => Err(()),
        }
    }
}

pub struct Config {
    tipo_de_arquivo: TipoDeArquivo,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Parâmetros insuficientes! Por favor, execute o comando com um tipo de arquivo como parâmetro. Os tipos válidos de arquivos são: empresas, estabelecimentos, cnaes, naturezas_juridicas, paises, municipios, qualificacoes_de_socios e ...")
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
