
use std::process;
use std::error::Error;
use std::io::Stdin;
use std::str::FromStr;
use std::io;

use csv::Reader;

use chrono::{NaiveDate};
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_15;
use serde_derive::Deserialize;
use bigdecimal::{BigDecimal};

use crate::types::TipoDeArquivo;
use crate::config::Config;
use crate::database::Database;
use crate::models::*;



// nos campos razao_social e ente_federativo_responsavel foi necessario o uso do serde_bytes
// porque os arquivos da receita estão no formato ISO-8859-15, e o Rust não aceita String
// que não seja no formato UTF-8
#[derive(Debug, Deserialize)]
struct EmpresaCsvRecord<'a> {
    cnpj_basico: String,
    #[serde(with = "serde_bytes")]
    razao_social: &'a [u8],
    natureza_juridica: i32,
    qualificacao_do_responsavel: i32,
    capital_social_da_empresa: String,
    porte_da_empresa: &'a [u8], // não pode ser i32, porque em alguns registros o campo está em branco e isto geraria um erro ao fazer o parse para integer.
    //gambiarra 
    #[serde(with = "serde_bytes")]
    ente_federativo_responsavel: &'a [u8]
}

#[derive(Debug, Deserialize)]
struct EstabelecimentoCsvRecord<'a> {
    cnpj_basico: String,
    cnpj_ordem: String,
    cnpj_dv: String,
    identificador_matriz_filial: String,
    #[serde(with = "serde_bytes")]
    nome_fantasia: &'a [u8],
    situacao_cadastral: Option<String>,
    data_situacao_cadastral: Option<String>,
    motivo_situacao_cadastral: Option<i32>,
    #[serde(with = "serde_bytes")]
    nome_da_cidade_no_exterior: &'a [u8],
    #[serde(deserialize_with = "csv::invalid_option")]
    pais: Option<i32>,
    data_de_inicio_da_atividade:Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    cnae_fiscal_principal: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    cnae_fiscal_secundaria: Option<String>,
    #[serde(with = "serde_bytes")]
    tipo_logradouro: &'a [u8],
    #[serde(with = "serde_bytes")]
    logradouro: &'a [u8],
    #[serde(with = "serde_bytes")]
    numero: &'a [u8],
    #[serde(with = "serde_bytes")]
    complemento: &'a [u8],
    #[serde(with = "serde_bytes")]
    bairro: &'a [u8],
    cep: Option<String>,
    uf: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    municipio: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd1: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone1: Option<i64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd2: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone2: Option<i64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd_fax: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone_fax: Option<i64>,
    #[serde(with = "serde_bytes")]
    correio_eletronico: &'a [u8],
    situacao_especial: Option<String>,
    data_situacao_especial: Option<String>,    
}

#[derive(Debug, Deserialize)]
struct DefaultCsvRecord<'a> {
    id: i32,
    #[serde(with="serde_bytes")]
    nome: &'a [u8],
}

fn naive_date_from_str(date_option: Option<String>) -> Option<NaiveDate> {


    let date_str = match date_option {
        Some(v) => v,
        None => return None
    };

    let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y%m%d");

    match parsed_date {
        Ok(v) => Some(v),
        Err(_e) => None
    }

}
pub struct Import {
    config: Config,
    db: Database
}

impl Import {
    pub fn new(config: Config) -> Import {

        let db = Database::new();

        Import { config, db }
    }

    pub fn run(&self) {
    
        let rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .from_reader(io::stdin());
    
        if let Err(err) = match self.config.tipo_de_arquivo() {
            TipoDeArquivo::Empresas => self.import_empresas(rdr),
            TipoDeArquivo::CNAES => self.import_default(rdr),
            TipoDeArquivo::Estabelecimentos => self.import_estabelecimentos(rdr),
            TipoDeArquivo::NaturezasJuridicas => self.import_default(rdr),
            TipoDeArquivo::QualificacoesDeSocios => self.import_default(rdr),
            TipoDeArquivo::Paises => self.import_default(rdr),
            TipoDeArquivo::Municipios => self.import_default(rdr),
            TipoDeArquivo::MotivosDeSituacoesCadastrais => self.import_default(rdr)
        } {
            println!("Erro ao executar: {}", err);
            process::exit(1);
        };
    }


    fn import_empresas(&self, mut rdr: Reader<Stdin>) -> Result<(), Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {

            let record: EmpresaCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));
            let razao_social = ISO_8859_15.decode(record.razao_social, DecoderTrap::Strict)?;
            let ente_federativo_responsavel = ISO_8859_15.decode(record.ente_federativo_responsavel, DecoderTrap::Strict)?;
            let porte_da_empresa = match std::str::from_utf8(&record.porte_da_empresa) {
                Ok(v) => v,
                Err(_e) => "ER",
            };

            let tbl_record = NewEmpresa {
                cnpj_basico: &record.cnpj_basico,
                razao_social: &razao_social,
                natureza_juridica: Some(record.natureza_juridica),
                qualificacao_do_responsavel: Some(record.qualificacao_do_responsavel),
                capital_social: Some(BigDecimal::from_str(&record.capital_social_da_empresa.replacen(",",".",1)).unwrap()), // arrumar a conversão aqui
                porte: Some(&porte_da_empresa),
                ente_federativo_responsavel: Some(&ente_federativo_responsavel)
            };
            self.db.upsert_empresa(&tbl_record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
        }
        Ok(())
    }

    fn import_estabelecimentos(&self, mut rdr: Reader<Stdin>) -> Result<(), Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {

            let record: EstabelecimentoCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));
            let nome_fantasia = Some(ISO_8859_15.decode(record.nome_fantasia, DecoderTrap::Strict).unwrap());
            let nome_da_cidade_no_exterior = Some(ISO_8859_15.decode(record.nome_da_cidade_no_exterior, DecoderTrap::Strict).unwrap());
            let logradouro = Some(ISO_8859_15.decode(record.logradouro, DecoderTrap::Strict).unwrap());
            let bairro = Some(ISO_8859_15.decode(record.bairro, DecoderTrap::Strict).unwrap());
            let complemento = Some(ISO_8859_15.decode(record.complemento, DecoderTrap::Strict).unwrap());
            let tipo_logradouro = Some(ISO_8859_15.decode(record.tipo_logradouro, DecoderTrap::Strict).unwrap());
            let numero = Some(ISO_8859_15.decode(record.numero, DecoderTrap::Strict).unwrap());
            let correio_eletronico = Some(ISO_8859_15.decode(record.correio_eletronico, DecoderTrap::Strict).unwrap());

            let tbl_record = NewEstabelecimento {
                cnpj_basico: &record.cnpj_basico,
                cnpj_ordem: &record.cnpj_ordem,
                cnpj_dv: &record.cnpj_dv,
                identificador_matriz_filial: &record.identificador_matriz_filial,
                nome_fantasia: nome_fantasia,
                situacao_cadastral: record.situacao_cadastral,
                data_situacao_cadastral: naive_date_from_str(record.data_situacao_cadastral),
                motivo_situacao_cadastral: record.motivo_situacao_cadastral,
                nome_da_cidade_no_exterior: nome_da_cidade_no_exterior,
                pais: record.pais,
                data_de_inicio_da_atividade: naive_date_from_str(record.data_de_inicio_da_atividade),
                cnae_fiscal_principal: record.cnae_fiscal_principal,
                cnae_fiscal_secundaria: record.cnae_fiscal_secundaria,
                tipo_logradouro: tipo_logradouro,
                logradouro: logradouro,
                numero: numero,
                complemento: complemento,
                bairro: bairro,
                cep: record.cep,
                uf: record.uf,
                municipio: record.municipio,
                ddd1: record.ddd1,
                telefone1: record.telefone1,
                ddd2: record.ddd2,
                telefone2: record.telefone2,
                ddd_fax: record.ddd_fax,
                telefone_fax: record.telefone_fax,
                correio_eletronico: correio_eletronico,
                situacao_especial: record.situacao_especial,
                data_situacao_especial: naive_date_from_str(record.data_situacao_especial)

            };
            self.db.upsert_estabelecimento(&tbl_record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
        }
        Ok(())
    }    

    fn import_default(&self, mut rdr: Reader<Stdin>) -> Result<(), Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();

        let tipo_arquivo = &self.config.tipo_de_arquivo();

        while rdr.read_byte_record(&mut raw_record)? {
            let csv_record: DefaultCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));
            let nome = ISO_8859_15.decode(csv_record.nome, DecoderTrap::Strict)?;

            match tipo_arquivo {
                TipoDeArquivo::Empresas => {
                    
                },
                TipoDeArquivo::CNAES => {
                    let tbl_record = NewCNAE {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_cnae(&tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                },
                TipoDeArquivo::Estabelecimentos => {

                },
                TipoDeArquivo::NaturezasJuridicas => {
                    let tbl_record = NewNaturezaJuridica {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_natureza_juridica( &tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                    
                },
                TipoDeArquivo::QualificacoesDeSocios => {
                    let tbl_record = NewQualificacaoDeSocio {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_qualificacoes_de_socios(&tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                    
                },
                TipoDeArquivo::Paises => {
                    let tbl_record = NewPais {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_paises( &tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                    
                },
                TipoDeArquivo::Municipios => {
                    let tbl_record = NewMunicipio {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_municipios( &tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                    
                },
                TipoDeArquivo::MotivosDeSituacoesCadastrais => {
                    let tbl_record = NewMotivoDeSituacaoCadastral {
                        id: csv_record.id,
                        nome: &nome,
                    };
        
                    self.db.upsert_motivo_de_situacao_cadastral(&tbl_record)
                        .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&tbl_record));
                },        
            };

            
        }    
        Ok(())
    }

}