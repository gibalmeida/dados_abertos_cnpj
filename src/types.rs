use std::str::FromStr;

pub enum TipoDeArquivo {
    Empresas,
    Estabelecimentos,
    CNAES,
    NaturezasJuridicas,
    QualificacoesDeSocios,
    Paises,
    Municipios,
    MotivosDeSituacoesCadastrais
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
            "motivos_de_situacoes_cadastrais" => Ok(TipoDeArquivo::MotivosDeSituacoesCadastrais),
            _      => Err(()),
        }
    }
}