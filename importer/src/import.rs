
use std::process;
use std::error::Error;
use std::io::Stdin;
use std::str::FromStr;
use std::io;

use csv::Reader;

use chrono::NaiveDate;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_15;
use serde_derive::Deserialize;
use bigdecimal::BigDecimal;

use crate::tipo_de_arquivo::TipoDeArquivo;
use crate::config::Config;
use data_models::database::Database;
use data_models::models::*;



// nos campos razao_social e ente_federativo_responsavel foi necessario o uso do serde_bytes
// porque os arquivos da receita estão no formato ISO-8859-15, e o Rust não aceita String
// que não seja no formato UTF-8
#[derive(Debug, Deserialize)]
struct EmpresaCsvRecord<'a> {
    cnpj_basico: String,
    #[serde(with = "serde_bytes")]
    razao_social: &'a [u8],
    natureza_juridica: u16,
    qualificacao_do_responsavel: u8,
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
    situacao_cadastral: u8,
    data_situacao_cadastral: Option<String>,
    motivo_situacao_cadastral: Option<u8>,
    #[serde(with = "serde_bytes")]
    nome_da_cidade_no_exterior: &'a [u8],
    #[serde(deserialize_with = "csv::invalid_option")]
    pais: Option<u16>,
    data_de_inicio_da_atividade:Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    cnae_fiscal_principal: Option<u32>,
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
    municipio: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd1: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone1: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd2: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone2: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ddd_fax: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefone_fax: Option<String>,
    #[serde(with = "serde_bytes")]
    correio_eletronico: &'a [u8],
    situacao_especial: Option<String>,
    data_situacao_especial: Option<String>,    
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
    
        let processing_result = match self.config.tipo_de_arquivo() {
            TipoDeArquivo::Empresas => self.import_empresas(rdr),
            TipoDeArquivo::CNAES => self.import_cnaes(rdr),
            TipoDeArquivo::Estabelecimentos => self.import_estabelecimentos(rdr),
            TipoDeArquivo::NaturezasJuridicas => self.import_naturezas_juridicas(rdr),
            TipoDeArquivo::QualificacoesDeSocios => self.import_qualificacoes_de_socios(rdr),
            TipoDeArquivo::Paises => self.import_paises(rdr),
            TipoDeArquivo::Municipios => self.import_municipios(rdr),
            TipoDeArquivo::MotivosDeSituacoesCadastrais => self.import_motivos_de_situacoes_cadastrais(rdr)
        };
        
        if let Err(err) = processing_result {
            println!("Erro ao executar: {}", err);
            process::exit(1);
        } else {
            println!("{} registros importados.", processing_result.unwrap());
        };



        self.db.commit();
    }


    fn import_empresas(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        const RECORDS_LIMIT: usize = 1000;
        let mut records: Vec<NewEmpresa> = Vec::with_capacity(RECORDS_LIMIT);

        while rdr.read_byte_record(&mut raw_record)? {

            let record: EmpresaCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));
            let razao_social = ISO_8859_15.decode(record.razao_social, DecoderTrap::Strict)?;
            let ente_federativo_responsavel = ISO_8859_15.decode(record.ente_federativo_responsavel, DecoderTrap::Strict)?;
            let porte_da_empresa = match std::str::from_utf8(record.porte_da_empresa) {
                Ok(v) => v.to_string(),
                Err(_e) => "ER".to_string(),
            };

            records.push(NewEmpresa {
                cnpj_basico: record.cnpj_basico,
                razao_social,
                natureza_juridica: Some(record.natureza_juridica),
                qualificacao_do_responsavel: Some(record.qualificacao_do_responsavel),
                capital_social: Some(BigDecimal::from_str(&record.capital_social_da_empresa.replacen(",",".",1)).unwrap()), // arrumar a conversão aqui
                porte: Some(porte_da_empresa),
                ente_federativo_responsavel: Some(ente_federativo_responsavel)
            });

            num_records+=1;

            if records.len() == RECORDS_LIMIT {
                self.db.insert_empresa(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de empresas!"));
                records.clear();
                println!("{} registros importados até agora.", &num_records);
            }

        }
        self.db.insert_empresa(&records)
            .expect(&format!("Erro ao inserir registros na tabela de empresas!"));

        Ok(num_records)
    }

    fn import_estabelecimentos(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        const RECORDS_LIMIT: usize = 1000;
        let mut records: Vec<NewEstabelecimento> = Vec::with_capacity(RECORDS_LIMIT);

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

            records.push( NewEstabelecimento {
                cnpj_basico: record.cnpj_basico,
                cnpj_ordem: record.cnpj_ordem,
                cnpj_dv: record.cnpj_dv,
                identificador_matriz_filial: record.identificador_matriz_filial,
                nome_fantasia,
                situacao_cadastral: record.situacao_cadastral,
                data_situacao_cadastral: naive_date_from_str(record.data_situacao_cadastral),
                motivo_situacao_cadastral: record.motivo_situacao_cadastral,
                nome_da_cidade_no_exterior,
                pais: record.pais,
                data_de_inicio_da_atividade: naive_date_from_str(record.data_de_inicio_da_atividade),
                cnae_fiscal_principal: record.cnae_fiscal_principal,
                cnae_fiscal_secundaria: record.cnae_fiscal_secundaria,
                tipo_logradouro,
                logradouro,
                numero,
                complemento,
                bairro,
                cep: record.cep,
                uf: record.uf,
                municipio: record.municipio,
                ddd1: record.ddd1,
                telefone1: record.telefone1,
                ddd2: record.ddd2,
                telefone2: record.telefone2,
                ddd_fax: record.ddd_fax,
                telefone_fax: record.telefone_fax,
                correio_eletronico,
                situacao_especial: record.situacao_especial,
                data_situacao_especial: naive_date_from_str(record.data_situacao_especial)

            });

            num_records += 1;

            if records.len() == RECORDS_LIMIT {
                self.db.insert_estabelecimento(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de estabelecimentos!"));
                records.clear();
                println!("{} registros importados até agora.", &num_records);
            }

        }

        self.db.insert_estabelecimento(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de estabelecimentos!"));

        Ok(num_records)
    }
    
    fn import_cnaes(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u32 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewCNAE {
                id,
                nome,
            };

            self.db.upsert_cnae(&record)
            .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }  
    
    fn import_naturezas_juridicas(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewNaturezaJuridica {
                id,
                nome,
            };

            self.db.upsert_natureza_juridica(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }

    fn import_qualificacoes_de_socios(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u8 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewQualificacaoDeSocio {
                id,
                nome,
            };

            self.db.upsert_qualificacoes_de_socios(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }

    fn import_paises(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewPais {
                id,
                nome,
            };

            self.db.upsert_paises(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }
    
    fn import_municipios(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewMunicipio {
                id,
                nome,
            };

            self.db.upsert_municipios(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }

    fn import_motivos_de_situacoes_cadastrais(&self, mut rdr: Reader<Stdin>) -> Result<usize, Box<dyn Error>> {
        let mut raw_record = csv::ByteRecord::new();
        let mut num_records = 0;

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u8 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewMotivoDeSituacaoCadastral {
                id,
                nome,
            };

            self.db.upsert_motivo_de_situacao_cadastral(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            num_records += 1;
            
        }
        Ok(num_records)
    }
}