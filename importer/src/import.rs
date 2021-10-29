
use std::io::{self, Write};
use std::error::Error;
use std::str::FromStr;
use std::time::Instant;

use csv::Reader;

use chrono::NaiveDate;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_15;
use serde_derive::Deserialize;
use bigdecimal::BigDecimal;
use zip::read::ZipFile;

use crate::tipo_de_arquivo::TipoDeArquivo;
use crate::config::Config;
use crate::database::Database;
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

#[derive(Debug, Deserialize)]
struct SocioCsvRecord<'a> {
    cnpj_basico: String,
    identificador_de_socio: u8,
    #[serde(with = "serde_bytes")]
    nome_ou_razao_social_do_socio: &'a [u8],
    cnpj_ou_cpf_do_socio: Option<String>,
    qualificacao_do_socio: u8,
    data_de_entrada_na_sociedade: String,
    pais_do_socio: Option<u16>,
    cpf_do_representante_legal: String,
    #[serde(with = "serde_bytes")]
    nome_do_representante_legal: &'a [u8],
    qualificacao_do_representante_legal: u8,
    faixa_etaria_do_socio: u8,    
}

#[derive(Debug, Deserialize)]
struct SimplesCsvRecord {
    cnpj_basico: String,
    opcao_pelo_simples: String,
    data_de_opcao_pelo_simples: Option<String>,
    data_de_exclusao_do_simples: Option<String>,
    opcao_pelo_mei: String,
    data_de_opcao_pelo_mei: Option<String>,
    data_de_exclusao_do_mei: Option<String>,
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
pub struct Import<'a> {
    config: &'a Config<'a>,
    db: Database<'a>,
    start_time: Instant,
    num_records: usize,
}

impl<'a> Import<'a> {
    pub fn new(config: &'a Config<'a>) -> Import<'a> {

        let db = Database::new(&config);
        let start_time = Instant::now();

        Import { config, db, start_time, num_records: 0 }
    }

    pub fn run(&mut self, file: ZipFile) -> Result<(), String>  {

        let filename = &*file.name().to_owned();

        if self.file_already_imported(filename) {
            println!("Como o arquivo já foi importando anteriormente, vamos pular ele. Utilize --force para forçar a importação novamente.");
            return Ok(()); // se for importação de um diretório, vai para o próximo aquivo; senão encerra.
        }
    
        let rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .from_reader(file);
    
        self.db.before_table_update();

        let mut import_table = |rdr| {
            match self.config.tipo_de_arquivo() {
            TipoDeArquivo::Empresas => self.import_empresas(rdr),
            TipoDeArquivo::Estabelecimentos => self.import_estabelecimentos(rdr),
            TipoDeArquivo::CNAES => self.import_cnaes(rdr),
            TipoDeArquivo::NaturezasJuridicas => self.import_naturezas_juridicas(rdr),
            TipoDeArquivo::QualificacoesDeSocios => self.import_qualificacoes_de_socios(rdr),
            TipoDeArquivo::Paises => self.import_paises(rdr),
            TipoDeArquivo::Municipios => self.import_municipios(rdr),
            TipoDeArquivo::MotivosDeSituacoesCadastrais => self.import_motivos_de_situacoes_cadastrais(rdr),
            TipoDeArquivo::Simples => self.import_simples(rdr),
            TipoDeArquivo::Socios => self.import_socios(rdr),
            }
        };
        
        match import_table(rdr) {
            Ok(()) => {

                self.db.after_table_update();

                let duration_in_seconds = self.duration_in_seconds();
                let arquivo_importado = NewArquivoImportado{
                    nome_do_arquivo: filename,
                    tabela: self.config.tipo_de_arquivo().table_name(),
                    tempo_decorrido_em_segundos: Some(duration_in_seconds),
                    registros_processados: self.num_records as u32,
                };

                self.db
                    .upsert_arquivo_importado(&arquivo_importado)
                    .expect(&format!("Erro ao inserir registros na tabela de arquivos importados!"));
                self.db.commit();

                if duration_in_seconds == 0 {
                    println!("{} registros importados em {} milissegundos: {} registros/segundo", self.num_records, self.duration_in_millis(), self.records_per_seconds() );
                } else {
                    println!("{} registros importados em {} segundos: {} registros/segundo", self.num_records, duration_in_seconds, self.records_per_seconds());
                }

            },
            Err(err) =>return Err(format!("Erro ao executar: {}", err))
        } 

        Ok(())
    }


    fn file_already_imported(&self, filename: &str) -> bool {

        match self.db.fetch_arquivo_importado(&filename) {
            Ok(_arquivo) => {
                match self.config.force() {
                    true => {
                        println!("O arquivo já foi importado anteriormente; mas, como o flag --force foi informado, vamos importá-lo novamente.");
                        false
                    },
                    _ => true
                    
                }
            },
            _ => false
 
        }

    }

    fn import_empresas<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();
        let mut records: Vec<NewEmpresa> = Vec::with_capacity(self.config.rows_per_insert());

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

            self.num_records+=1;

            if records.len() == self.config.rows_per_insert() {
                self.db.upsert_empresa(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de empresas!"));
                records.clear();
                self.show_progress();
            }

        }
        self.db.upsert_empresa(&records)
            .expect(&format!("Erro ao inserir registros na tabela de empresas!"));
                
        Ok(())
    }

    fn import_estabelecimentos<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        let mut records: Vec<NewEstabelecimento> = Vec::with_capacity(self.config.rows_per_insert());

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

            self.num_records += 1;

            if records.len() == self.config.rows_per_insert() {
                self.db.upsert_estabelecimento(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de estabelecimentos!"));
                records.clear();
                self.show_progress();
            }

        }

        self.db.upsert_estabelecimento(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de estabelecimentos!"));

        Ok(())
    }
    
    fn import_socios<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        let mut records: Vec<NewSocio> = Vec::with_capacity(self.config.rows_per_insert());

        while rdr.read_byte_record(&mut raw_record)? {

            let record: SocioCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));

            records.push( NewSocio {
                cnpj_basico: record.cnpj_basico,
                identificador_de_socio: record.identificador_de_socio,
                nome_ou_razao_social_do_socio: ISO_8859_15.decode(record.nome_ou_razao_social_do_socio, DecoderTrap::Strict).unwrap(),
                cnpj_ou_cpf_do_socio: record.cnpj_ou_cpf_do_socio,
                qualificacao_do_socio: record.qualificacao_do_socio,
                data_de_entrada_na_sociedade: NaiveDate::parse_from_str(&record.data_de_entrada_na_sociedade, "%Y%m%d").unwrap(),
                pais_do_socio: record.pais_do_socio,
                cpf_do_representante_legal: record.cpf_do_representante_legal,
                nome_do_representante_legal: ISO_8859_15.decode(record.nome_do_representante_legal, DecoderTrap::Strict).unwrap(),
                qualificacao_do_representante_legal: record.qualificacao_do_representante_legal,
                faixa_etaria_do_socio: record.faixa_etaria_do_socio,  

            });

            self.num_records += 1;

            if records.len() == self.config.rows_per_insert() {
                self.db.upsert_socio(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de socios!"));
                records.clear();
                self.show_progress();
            }

        }

        self.db.upsert_socio(&records)
                    .expect(&format!("Erro ao inserir registros na tabela de socios!"));

        Ok(())
    }

    fn import_simples<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        let mut records: Vec<NewSimples> = Vec::with_capacity(self.config.rows_per_insert());

        while rdr.read_byte_record(&mut raw_record)? {

            let record: SimplesCsvRecord = raw_record.deserialize(None)
                .expect(&format!("Erro ao deserializar o seguinte registro: {:?}", raw_record));

            records.push( NewSimples {
                cnpj_basico: record.cnpj_basico,
                opcao_pelo_simples: record.opcao_pelo_simples,
                data_de_opcao_pelo_simples: naive_date_from_str(record.data_de_opcao_pelo_simples),
                data_de_exclusao_do_simples: naive_date_from_str(record.data_de_exclusao_do_simples),
                opcao_pelo_mei: record.opcao_pelo_mei,
                data_de_opcao_pelo_mei: naive_date_from_str(record.data_de_opcao_pelo_mei),
                data_de_exclusao_do_mei: naive_date_from_str(record.data_de_exclusao_do_mei),
            });

            self.num_records += 1;

            if records.len() == self.config.rows_per_insert() {
                self.db.upsert_simples(&records)
                    .expect(&format!("Erro ao inserir registros na tabela do simples!"));
                records.clear();
                self.show_progress();
            }

        }

        self.db.upsert_simples(&records)
                    .expect(&format!("Erro ao inserir registros na tabela do simples!"));

        Ok(())
    }

    fn import_cnaes<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u32 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewCNAE {
                id,
                nome,
            };

            self.db.upsert_cnae(&record)
            .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }  
    
    fn import_naturezas_juridicas<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewNaturezaJuridica {
                id,
                nome,
            };

            self.db.upsert_natureza_juridica(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }

    fn import_qualificacoes_de_socios<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u8 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewQualificacaoDeSocio {
                id,
                nome,
            };

            self.db.upsert_qualificacoes_de_socios(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }

    fn import_paises<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewPais {
                id,
                nome,
            };

            self.db.upsert_paises(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }
    
    fn import_municipios<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u16 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewMunicipio {
                id,
                nome,
            };

            self.db.upsert_municipios(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }

    fn import_motivos_de_situacoes_cadastrais<R>(&mut self, mut rdr: Reader<R>) -> Result<(), Box<dyn Error>> where R: io::Read, {
        let mut raw_record = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut raw_record)? {
            let id: u8 = std::str::from_utf8(&raw_record[0]).unwrap().parse().unwrap();
            let nome =  ISO_8859_15.decode(&raw_record[1], DecoderTrap::Strict)?;
            let record = NewMotivoDeSituacaoCadastral {
                id,
                nome,
            };

            self.db.upsert_motivo_de_situacao_cadastral(&record)
                .expect(&format!("Erro ao inserir o seguinte registro: {:?}",&record));

            self.num_records += 1;
            self.show_progress();
            
        }
        Ok(())
    }

    fn show_progress(&self) {
        if self.config.verbose() {
            // os espaços no final servem para sobrepor na reimpressão do texto, já que o tamanho é variável
            print!("{} registros importados até agora. {} registros/segundo.                                                           \r", self.num_records, self.records_per_seconds());
            io::stdout().flush().unwrap();
        }
    }

    fn duration_in_seconds(&self) -> u64 {
        Instant::now().duration_since(self.start_time).as_secs()
    }

    fn duration_in_millis(&self) -> u128 {
        Instant::now().duration_since(self.start_time).as_millis()
    }

    fn records_per_seconds(&self) -> u64 {
        let duration_in_seconds = self.duration_in_seconds();
        if duration_in_seconds == 0 {
            return self.num_records as u64
        };

        self.num_records as u64 / duration_in_seconds
    }
}
