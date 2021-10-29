use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use super::schema::{empresas,estabelecimentos,cnaes,naturezas_juridicas,municipios,paises,qualificacoes_de_socios,motivos_de_situacoes_cadastrais,arquivos_importados,socios,simples};

#[derive(Identifiable, Queryable, Serialize)]
#[primary_key(cnpj_basico)]
pub struct Empresa {
    pub cnpj_basico: String,
    pub razao_social: String,
    pub natureza_juridica: Option<u16>,
    pub qualificacao_do_responsavel: Option<u8>,
    pub capital_social: Option<BigDecimal>,
    pub porte: Option<String>,
    pub ente_federativo_responsavel: Option<String>
}

#[derive(Debug,Insertable)]
#[table_name="empresas"]
pub struct NewEmpresa {
    pub cnpj_basico: String,
    pub razao_social: String,
    pub natureza_juridica: Option<u16>,
    pub qualificacao_do_responsavel: Option<u8>,
    pub capital_social: Option<BigDecimal>,
    pub porte: Option<String>,
    pub ente_federativo_responsavel: Option<String>
}

#[derive(Identifiable,Queryable, Serialize)]
#[table_name="naturezas_juridicas"]
pub struct NaturezaJuridica {
    pub id: u16,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="naturezas_juridicas"]
pub struct NewNaturezaJuridica {
    pub id: u16,
    pub nome: String,
}

#[derive(Identifiable,Queryable, Serialize)]
#[table_name="qualificacoes_de_socios"]
pub struct QualificacaoDeSocio {
    pub id: u8,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="qualificacoes_de_socios"]
pub struct NewQualificacaoDeSocio {
    pub id: u8,
    pub nome: String,
}

#[derive(Identifiable,Queryable, Serialize)]
#[table_name="cnaes"]
pub struct CNAE {
    pub id: u32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="cnaes"] 
pub struct NewCNAE {
    pub id: u32,
    pub nome: String,
}

#[derive(Identifiable,Queryable, Serialize)]
#[table_name="paises"]
pub struct Pais {
    pub id: u16,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="paises"] 
pub struct NewPais {
    pub id: u16,
    pub nome: String,
}

#[derive(Identifiable,Queryable, Serialize)]
#[table_name="motivos_de_situacoes_cadastrais"]
pub struct MotivoDeSituacaoCadastral {
    pub id: u8,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="motivos_de_situacoes_cadastrais"] 
pub struct NewMotivoDeSituacaoCadastral {
    pub id: u8,
    pub nome: String,
}


#[derive(Identifiable,Queryable, Serialize)]
pub struct Municipio {
    pub id: u16,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="municipios"] 
pub struct NewMunicipio {
    pub id: u16,
    pub nome: String,
}
#[derive(Identifiable, Queryable, Associations, Serialize)]
#[primary_key(cnpj_basico,cnpj_ordem,cnpj_dv)]
#[belongs_to(Empresa, foreign_key="cnpj_basico")]
pub struct Estabelecimento {
    pub cnpj_basico: String,
    pub cnpj_ordem: String,
    pub cnpj_dv: String,
    pub identificador_matriz_filial: String,
    pub nome_fantasia: Option<String>,
    pub situacao_cadastral: u8,
    pub data_situacao_cadastral: Option<NaiveDate>,
    pub motivo_situacao_cadastral: Option<u8>,
    pub nome_da_cidade_no_exterior: Option<String>,
    pub pais: Option<u16>,
    pub data_de_inicio_da_atividade: Option<NaiveDate>,
    pub cnae_fiscal_principal: Option<u32>,
    pub cnae_fiscal_secundaria: Option<String>,
    pub tipo_logradouro: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: Option<String>,
    pub uf: Option<String>,
    pub municipio: Option<u16>,
    pub ddd1: Option<String>,
    pub telefone1: Option<String>,
    pub ddd2: Option<String>,
    pub telefone2: Option<String>,
    pub ddd_fax: Option<String>,
    pub telefone_fax: Option<String>,
    pub correio_eletronico: Option<String>,
    pub situacao_especial: Option<String>,
    pub data_situacao_especial: Option<NaiveDate>,
}

#[derive(Debug,Insertable)]
#[table_name="estabelecimentos"]
pub struct NewEstabelecimento {
    pub cnpj_basico: String,
    pub cnpj_ordem: String,
    pub cnpj_dv: String,
    pub identificador_matriz_filial: String,
    pub nome_fantasia: Option<String>,
    pub situacao_cadastral: u8,
    pub data_situacao_cadastral: Option<NaiveDate>,
    pub motivo_situacao_cadastral: Option<u8>,
    pub nome_da_cidade_no_exterior: Option<String>,
    pub pais: Option<u16>,
    pub data_de_inicio_da_atividade: Option<NaiveDate>,
    pub cnae_fiscal_principal: Option<u32>,
    pub cnae_fiscal_secundaria: Option<String>,
    pub tipo_logradouro: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: Option<String>,
    pub uf: Option<String>,
    pub municipio: Option<u16>,
    pub ddd1: Option<String>,
    pub telefone1: Option<String>,
    pub ddd2: Option<String>,
    pub telefone2: Option<String>,
    pub ddd_fax: Option<String>,
    pub telefone_fax: Option<String>,
    pub correio_eletronico: Option<String>,
    pub situacao_especial: Option<String>,
    pub data_situacao_especial: Option<NaiveDate>,    
}

#[derive(Queryable)]
pub struct ArquivoImportado {
    pub nome_do_arquivo: String,
    pub tabela: String,
    pub registros_processados: u32,
    pub tempo_decorrido_em_segundos: Option<u64>,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="arquivos_importados"]
pub struct NewArquivoImportado<'a> {
    pub nome_do_arquivo: &'a str,
    pub tabela: &'a str,
    pub registros_processados: u32,
    pub tempo_decorrido_em_segundos: Option<u64>
}

#[derive(Queryable)]
pub struct Socio {
    pub id: u32,
    pub cnpj_basico: String,
    pub identificador_de_socio: u8,
    pub nome_ou_razao_social_do_socio: String,
    pub cnpj_ou_cpf_do_socio: Option<String>,
    pub qualificacao_do_socio: u8,
    pub data_de_entrada_na_sociedade: NaiveDate,
    pub pais_do_socio: Option<u16>,
    pub cpf_do_representante_legal: String,
    pub nome_do_representante_legal: String,
    pub qualificacao_do_representante_legal: u8,
    pub faixa_etaria_do_socio: u8
}

#[derive(Debug,Insertable)]
#[table_name="socios"]
pub struct NewSocio {
    pub cnpj_basico: String,
    pub identificador_de_socio: u8,
    pub nome_ou_razao_social_do_socio: String,
    pub cnpj_ou_cpf_do_socio: Option<String>,
    pub qualificacao_do_socio: u8,
    pub data_de_entrada_na_sociedade: NaiveDate,
    pub pais_do_socio: Option<u16>,
    pub cpf_do_representante_legal: String,
    pub nome_do_representante_legal: String,
    pub qualificacao_do_representante_legal: u8,
    pub faixa_etaria_do_socio: u8,    
}

#[derive(Queryable)]
pub struct FaixaEtaria {
    pub id: u8,
    pub nome: Option<String>,
}

#[derive(Queryable)]
pub struct Simples {
    pub cnpj_basico: String,
    pub opcao_pelo_simples: String,
    pub data_de_opcao_pelo_simples: Option<NaiveDate>,
    pub data_de_exclusao_do_simples: Option<NaiveDate>,
    pub opcao_pelo_mei: String,
    pub data_de_opcao_pelo_mei: Option<NaiveDate>,
    pub data_de_exclusao_do_mei: Option<NaiveDate>,    
}
 
#[derive(Debug,Insertable)]
#[table_name="simples"]
pub struct NewSimples {
    pub cnpj_basico: String,
    pub opcao_pelo_simples: String,
    pub data_de_opcao_pelo_simples: Option<NaiveDate>,
    pub data_de_exclusao_do_simples: Option<NaiveDate>,
    pub opcao_pelo_mei: String,
    pub data_de_opcao_pelo_mei: Option<NaiveDate>,
    pub data_de_exclusao_do_mei: Option<NaiveDate>,
}