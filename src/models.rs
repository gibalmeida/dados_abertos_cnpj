use bigdecimal::BigDecimal;
use chrono::{NaiveDate};

use super::schema::{empresas, estabelecimentos, naturezas_juridicas, qualificacoes_de_socios, cnaes, paises, municipios, motivos_de_situacoes_cadastrais};

#[derive(Queryable)]
pub struct Empresa {
    pub cnpj_basico: String,
    pub razao_social: String,
    pub natureza_juridica: i32,
    pub qualificacao_do_responsavel: i32,
    pub capital_social: BigDecimal,
    pub porte: String,
    pub ente_federativo_responsavel: String
}

#[derive(Debug,Insertable)]
#[table_name="empresas"]
pub struct NewEmpresa<'a> {
    pub cnpj_basico: &'a str,
    pub razao_social: &'a str,
    pub natureza_juridica: Option<i32>,
    pub qualificacao_do_responsavel: Option<i32>,
    pub capital_social: Option<BigDecimal>,
    pub porte: Option<&'a str>,
    pub ente_federativo_responsavel: Option<&'a str>
}

#[derive(Queryable)]
pub struct NaturezaJuridica {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="naturezas_juridicas"]
pub struct NewNaturezaJuridica<'a> {
    pub id: i32,
    pub nome: &'a str,
}

#[derive(Queryable)]
pub struct QualificacaoDeSocio {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="qualificacoes_de_socios"]
pub struct NewQualificacaoDeSocio<'a> {
    pub id: i32,
    pub nome: &'a str,
}

#[derive(Queryable)]
pub struct CNAE {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="cnaes"] 
pub struct NewCNAE<'a> {
    pub id: i32,
    pub nome: &'a str,
}

#[derive(Queryable)]
pub struct Pais {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="paises"] 
pub struct NewPais<'a> {
    pub id: i32,
    pub nome: &'a str,
}

#[derive(Queryable)]
pub struct MotivoDeSituacaoCadastral {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="motivos_de_situacoes_cadastrais"] 
pub struct NewMotivoDeSituacaoCadastral<'a> {
    pub id: i32,
    pub nome: &'a str,
}


#[derive(Queryable)]
pub struct Municipio {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug,Insertable)]
#[table_name="municipios"] 
pub struct NewMunicipio<'a> {
    pub id: i32,
    pub nome: &'a str,
}
#[derive(Queryable)]
pub struct Estabelecimento {
    pub cnpj_basico: String,
    pub cnpj_ordem: String,
    pub cnpj_dv: String,
    pub identificador_matriz_filial: char,
    pub nome_fantasia: String,
    pub situacao_cadastral: char,
    pub data_situacao_cadastral: String,
    pub motivo_situacao_cadastral: char,
    pub nome_da_cidade_no_exterior: String,
    pub pais: i32,
    pub data_de_inicio_da_atividade: String,
    pub cnae_fiscal_principal: i32,
    pub cnae_fiscal_secundaria: i32,
    pub tipo_logradouro: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String,
    pub bairro: String,
    pub cep: String,
    pub uf: String,
    pub municipio: i32,
    pub ddd1: i32,
    pub telefone1: BigDecimal,
    pub ddd2: i32,
    pub telefone2: BigDecimal,
    pub ddd_fax: i32,
    pub telefone_fax: BigDecimal,
    pub correio_eletronico: String,
    pub situacao_especial: String,
    pub data_situacao_especial: String,    
}

#[derive(Debug,Insertable)]
#[table_name="estabelecimentos"]
pub struct NewEstabelecimento<'a> {
    pub cnpj_basico: &'a str,
    pub cnpj_ordem: &'a str,
    pub cnpj_dv: &'a str,
    pub identificador_matriz_filial: &'a str,
    pub nome_fantasia: Option<String>,
    pub situacao_cadastral: Option<String>,
    pub data_situacao_cadastral: Option<NaiveDate>,
    pub motivo_situacao_cadastral: Option<String>,
    pub nome_da_cidade_no_exterior: Option<String>,
    pub pais: Option<i32>,
    pub data_de_inicio_da_atividade: Option<NaiveDate>,
    pub cnae_fiscal_principal: Option<i32>,
    pub cnae_fiscal_secundaria: Option<i32>,
    pub tipo_logradouro: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: Option<String>,
    pub uf: Option<String>,
    pub municipio: Option<i32>,
    pub ddd1: Option<i32>,
    pub telefone1: Option<i64>,
    pub ddd2: Option<i32>,
    pub telefone2: Option<i64>,
    pub ddd_fax: Option<i32>,
    pub telefone_fax: Option<i64>,
    pub correio_eletronico: Option<String>,
    pub situacao_especial: Option<String>,
    pub data_situacao_especial: Option<NaiveDate>,    
}
