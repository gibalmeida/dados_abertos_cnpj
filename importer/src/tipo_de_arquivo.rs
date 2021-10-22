use std::str::FromStr;

pub enum TipoDeArquivo {
    Empresas,
    Estabelecimentos,
    CNAES,
    NaturezasJuridicas,
    QualificacoesDeSocios,
    Paises,
    Municipios,
    MotivosDeSituacoesCadastrais,
    Simples,
    Socios,
}

impl FromStr for TipoDeArquivo {

    type Err = ();

    fn from_str(filename: &str) -> Result<TipoDeArquivo, Self::Err> {
        

        if filename.contains(".SIMPLES.CSV") {
            return Ok(TipoDeArquivo::Simples);
        } else if filename.contains(".CNAECSV") {
            return Ok(TipoDeArquivo::CNAES);
        } else if filename.contains(".MOTICSV") {
            return Ok(TipoDeArquivo::MotivosDeSituacoesCadastrais);
        } else if filename.contains(".MUNICCSV") {
            return Ok(TipoDeArquivo::Municipios);
        } else if filename.contains(".NATJUCSV") {
            return Ok(TipoDeArquivo::NaturezasJuridicas);
        } else if filename.contains(".PAISCSV") {
            return Ok(TipoDeArquivo::Paises);
        } else if filename.contains(".QUALSCSV") {
            return Ok(TipoDeArquivo::QualificacoesDeSocios);
        } else if filename.contains(".EMPRECSV") {
            return Ok(TipoDeArquivo::Empresas);
        } else if filename.contains(".ESTABELE") {
            return Ok(TipoDeArquivo::Estabelecimentos);
        } else if filename.contains(".SOCIOCSV") {
            return Ok(TipoDeArquivo::Socios);
        }
        
        Err(())
    }
}

impl TipoDeArquivo {
    pub fn table_name(&self) -> &str {
        match &self {
            TipoDeArquivo::CNAES => "cnaes",
            TipoDeArquivo::Empresas => "empresas",
            TipoDeArquivo::Estabelecimentos => "estabelecimentos",
            TipoDeArquivo::MotivosDeSituacoesCadastrais => "motivos_de_situacoes_cadastrais",
            TipoDeArquivo::Municipios => "municipios",
            TipoDeArquivo::NaturezasJuridicas => "naturezas_juridicas",
            TipoDeArquivo::Paises => "paises",
            TipoDeArquivo::QualificacoesDeSocios => "qualificacoes_de_socios",
            TipoDeArquivo::Simples => "simples",
            TipoDeArquivo::Socios => "socios",            
        }
    }
}