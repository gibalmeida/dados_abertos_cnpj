extern crate dotenv;

use chrono::NaiveDate;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use juniper::{EmptyMutation, EmptySubscription, FieldError, RootNode};

use bigdecimal::{BigDecimal, ToPrimitive};

#[derive(Queryable)]
struct CNAE {
    id: u32,
    nome: String,
}

#[graphql_object(description = "CNAE - Classificação Nacional de Atividade Econômica")]
impl CNAE {
    pub fn id(&self) -> f64 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct NaturezaJuridica {
    id: u16,
    nome: String,
}

#[juniper::graphql_object(description = "Natureza Jurídica")]
impl NaturezaJuridica {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct Municipio {
    id: u16,
    nome: String,
}

#[graphql_object(description = "Município")]
impl Municipio {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct Pais {
    id: u16,
    nome: String,
}

#[graphql_object(description = "País")]
impl Pais {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct QualificacaoDeSocio {
    id: u8,
    nome: String,
}

#[graphql_object(description = "Qualificação de Sócio")]
impl QualificacaoDeSocio {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct MotivoDeSituacaoCadastral {
    id: u8,
    nome: String,
}

#[graphql_object(description = "Motivo de Situação Cadastral")]
impl MotivoDeSituacaoCadastral {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }
}

#[derive(Queryable)]
struct FaixaEtaria {
    id: u8,
    nome: Option<String>,
}

#[graphql_object(description = "Faixa Etária")]
impl FaixaEtaria {
    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn nome(&self) -> &Option<String> {
        &self.nome
    }
}

#[derive(Queryable)]
struct Socio {
    id: u32,
    cnpj_basico: String,
    identificador_de_socio: u8,
    nome_ou_razao_social_do_socio: String,
    cnpj_ou_cpf_do_socio: Option<String>,
    qualificacao_do_socio: u8,
    data_de_entrada_na_sociedade: NaiveDate,
    pais_do_socio: Option<u16>,
    cpf_do_representante_legal: String,
    nome_do_representante_legal: String,
    qualificacao_do_representante_legal: u8,
    faixa_etaria_do_socio: u8
}

#[juniper::graphql_object(context = Context, description="Sócio de uma empresa do CNPJ")]
impl Socio {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn cnpj_basico(&self) -> &String {
        &self.cnpj_basico
    }

    pub fn identificador_de_socio(&self) -> &str {
        match self.identificador_de_socio {
            1 => "Pessoa Jurídica",
            2 => "Pessoa Física",
            3 => "Estrangeiro",
            _ => "",
        }        
    }

    pub fn nome_ou_razao_social_do_socio(&self) -> &String {
        &self.nome_ou_razao_social_do_socio
    }

    pub fn cnpj_ou_cpf_do_socio(&self) -> &Option<String> {
        &self.cnpj_ou_cpf_do_socio
    }

    pub fn qualificacao_de_socio(&self, context: &Context) -> Result<Option<QualificacaoDeSocio>, FieldError> {
        use data_models::schema::qualificacoes_de_socios;
        let connection = context.pool.clone().get()?;

        Ok(Some(
            qualificacoes_de_socios::table
                .filter(qualificacoes_de_socios::id.eq(&self.qualificacao_do_socio))
                .first::<QualificacaoDeSocio>(&*connection)?,
        ))
    }

    pub fn data_de_entrada_na_sociedade(&self) -> &NaiveDate {
        &self.data_de_entrada_na_sociedade
    }

    pub fn pais_do_socio(&self, context: &Context) -> Result<Option<Pais>, FieldError> {
        match self.pais_do_socio {
            Some(v) => {
                use data_models::schema::paises;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    paises::table
                        .filter(paises::id.eq(v))
                        .first::<Pais>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn cpf_do_representante_legal(&self) -> &String {
        &self.cpf_do_representante_legal
    }

    pub fn nome_do_representante_legal(&self) -> &String {
        &self.nome_do_representante_legal
    }

    pub fn qualificacao_do_representante_legal(&self, context: &Context) -> Result<Option<QualificacaoDeSocio>, FieldError> {
        use data_models::schema::qualificacoes_de_socios;
        let connection = context.pool.clone().get()?;

        Ok(Some(
            qualificacoes_de_socios::table
                .filter(qualificacoes_de_socios::id.eq(&self.qualificacao_do_representante_legal))
                .first::<QualificacaoDeSocio>(&*connection)?,
        ))
    }
    

    pub fn faixa_etaria_do_socio(&self, context: &Context) -> Result<Option<FaixaEtaria>, FieldError> {
        use data_models::schema::faixas_etarias;
        let connection = context.pool.clone().get()?;

        Ok(Some(
            faixas_etarias::table
                .filter(faixas_etarias::id.eq(&self.faixa_etaria_do_socio))
                .first::<FaixaEtaria>(&*connection)?,
        ))
    }}



#[derive(Queryable)]
struct Simples {
    cnpj_basico: String,
    opcao_pelo_simples: String,
    data_de_opcao_pelo_simples: Option<NaiveDate>,
    data_de_exclusao_do_simples: Option<NaiveDate>,
    opcao_pelo_mei: String,
    data_de_opcao_pelo_mei: Option<NaiveDate>,
    data_de_exclusao_do_mei: Option<NaiveDate>,    
}

#[juniper::graphql_object(context = Context, description="Dados referente ao SIMPLES ou MEI")]
impl Simples {
    pub fn cnpj_basico(&self) -> &String {
        &self.cnpj_basico
    }

    pub fn opcao_pelo_simples(&self) -> &str {
        match &self.opcao_pelo_simples[..] {
            "S" => "Sim",
            "N" => "Não",
            _ => "Outros"
        }
    }

    pub fn data_de_opcao_pelo_simples(&self) -> &Option<NaiveDate> {
        &self.data_de_opcao_pelo_simples
    }

    pub fn data_de_exclusao_do_simples(&self) -> &Option<NaiveDate> {
        &self.data_de_exclusao_do_simples
    }

    pub fn opcao_pelo_mei(&self) -> &str {
        match &self.opcao_pelo_mei[..] {
            "S" => "Sim",
            "N" => "Não",
            _ => "Outros"
        }
    }

    pub fn data_de_opcao_pelo_mei(&self) -> &Option<NaiveDate> {
        &self.data_de_opcao_pelo_mei
    }

    pub fn data_de_exclusao_do_mei(&self) -> &Option<NaiveDate> {
        &self.data_de_exclusao_do_mei
    }

}
#[derive(Queryable)]
// #[primary_key(cnpj_basico)]
struct Empresa {
    cnpj_basico: String,
    razao_social: String,
    natureza_juridica: Option<u16>,
    qualificacao_do_responsavel: Option<u8>,
    capital_social: Option<BigDecimal>,
    porte: Option<String>,
    ente_federativo_responsavel: Option<String>,
}

#[juniper::graphql_object(context = Context, description="Uma Empresa registrada no CNPJ (Cadastro Nacional de Pessoas Jurídicas)")]
impl Empresa {
    pub fn cnpj_basico(&self) -> &String {
        &self.cnpj_basico
    }

    pub fn razao_social(&self) -> &String {
        &self.razao_social
    }

    pub fn natureza_juridica(
        &self,
        context: &Context,
    ) -> Result<Option<NaturezaJuridica>, FieldError> {
        match self.natureza_juridica {
            Some(v) => {
                use data_models::schema::naturezas_juridicas;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    naturezas_juridicas::table
                        .filter(naturezas_juridicas::id.eq(v))
                        .first::<NaturezaJuridica>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn qualificacao_do_responsavel(
        &self,
        context: &Context,
    ) -> Result<Option<QualificacaoDeSocio>, FieldError> {
        if let Some(v) = self.qualificacao_do_responsavel {
            use data_models::schema::qualificacoes_de_socios;
            let connection = context.pool.clone().get()?;
            Ok(Some(
                qualificacoes_de_socios::table
                    .filter(qualificacoes_de_socios::id.eq(v))
                    .first::<QualificacaoDeSocio>(&*connection)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn capital_social(&self) -> Option<f64> {
        self.capital_social.as_ref()?.to_f64()
    }

    pub fn porte(&self) -> &Option<String> {
        &self.porte
    }

    pub fn ente_federativo_responsavel(&self) -> &Option<String> {
        &self.ente_federativo_responsavel
    }

    pub fn socios(&self, context: &Context) -> Result<Vec<Socio>, FieldError> {
        use data_models::schema::socios;
        let connection = context.pool.clone().get()?;

        Ok(socios::table
            .filter(socios::cnpj_basico.eq(&self.cnpj_basico))
            .load::<Socio>(&*connection)?)
    }

    pub fn estabelecimentos(&self, context: &Context) -> Result<Vec<Estabelecimento>, FieldError> {
        use data_models::schema::estabelecimentos;
        let connection = context.pool.clone().get()?;

        Ok(estabelecimentos::table
            .filter(estabelecimentos::cnpj_basico.eq(&self.cnpj_basico))
            .load::<Estabelecimento>(&*connection)?)
    }
    
    pub fn simples(&self, context: &Context) -> Result<Simples, FieldError> {
        use data_models::schema::simples;
        let connection = context.pool.clone().get()?;

        Ok(simples::table
            .filter(simples::cnpj_basico.eq(&self.cnpj_basico))
            .first::<Simples>(&*connection)?)
    }}

#[derive(Queryable)]
struct Estabelecimento {
    cnpj_basico: String,
    cnpj_ordem: String,
    cnpj_dv: String,
    identificador_matriz_filial: String,
    nome_fantasia: Option<String>,
    situacao_cadastral: u8,
    data_situacao_cadastral: Option<NaiveDate>,
    motivo_situacao_cadastral: Option<u8>,
    nome_da_cidade_no_exterior: Option<String>,
    pais: Option<u16>,
    data_de_inicio_da_atividade: Option<NaiveDate>,
    cnae_fiscal_principal: Option<u32>,
    cnae_fiscal_secundaria: Option<String>,
    tipo_logradouro: Option<String>,
    logradouro: Option<String>,
    numero: Option<String>,
    complemento: Option<String>,
    bairro: Option<String>,
    cep: Option<String>,
    uf: Option<String>,
    municipio: Option<u16>,
    ddd1: Option<String>,
    telefone1: Option<String>,
    ddd2: Option<String>,
    telefone2: Option<String>,
    ddd_fax: Option<String>,
    telefone_fax: Option<String>,
    correio_eletronico: Option<String>,
    situacao_especial: Option<String>,
    data_situacao_especial: Option<NaiveDate>,
}

#[graphql_object(context = Context, description = "Um Estabelecimento pertencente a uma Empresa")]
impl Estabelecimento {
    pub fn cnpj_basico(&self) -> &String {
        &self.cnpj_basico
    }

    pub fn cnpj_ordem(&self) -> &String {
        &self.cnpj_ordem
    }

    pub fn cnpj_dv(&self) -> &String {
        &self.cnpj_dv
    }

    pub fn identificador_matriz_filial(&self) -> &str {
        match self.identificador_matriz_filial.as_str() {
            "1" => "Matriz",
            "2" => "Filial",
            _ => "",
        }
    }

    pub fn nome_fantasia(&self) -> &Option<String> {
        &self.nome_fantasia
    }

    pub fn situacao_cadastral(&self) -> SituacaoCadastral {
        match self.situacao_cadastral {
            1 => SituacaoCadastral::NULA,
            2 => SituacaoCadastral::ATIVA,
            3 => SituacaoCadastral::SUSPENSA,
            4 => SituacaoCadastral::INAPTA,
            8 => SituacaoCadastral::BAIXADA,
            _ => SituacaoCadastral::DESCONHECIDA,
        }
    }

    pub fn data_situacao_cadastral(&self) -> &Option<NaiveDate> {
        &self.data_situacao_cadastral
    }

    pub fn motivo_situacao_cadastral(
        &self,
        context: &Context,
    ) -> Result<Option<MotivoDeSituacaoCadastral>, FieldError> {
        match self.motivo_situacao_cadastral {
            Some(v) => {
                use data_models::schema::motivos_de_situacoes_cadastrais;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    motivos_de_situacoes_cadastrais::table
                        .filter(motivos_de_situacoes_cadastrais::id.eq(v))
                        .first::<MotivoDeSituacaoCadastral>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn nome_da_cidade_no_exterior(&self) -> &Option<String> {
        &self.nome_da_cidade_no_exterior
    }

    pub fn pais(&self, context: &Context) -> Result<Option<Pais>, FieldError> {
        match self.pais {
            Some(v) => {
                use data_models::schema::paises;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    paises::table
                        .filter(paises::id.eq(v))
                        .first::<Pais>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn data_de_inicio_da_atividade(&self) -> &Option<NaiveDate> {
        &self.data_de_inicio_da_atividade
    }

    pub fn cnae_fiscal_principal(&self, context: &Context) -> Result<Option<CNAE>, FieldError> {
        match self.cnae_fiscal_principal {
            Some(v) => {
                use data_models::schema::cnaes;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    cnaes::table
                        .filter(cnaes::id.eq(v))
                        .first::<CNAE>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn cnaes_fiscais_secundarias(&self, context: &Context) -> Result<Vec<CNAE>, FieldError> {
        use data_models::schema::cnaes;
        let connection = context.pool.clone().get()?;

        let cnaes_fiscais_secundarias: Vec<u32> = match &self.cnae_fiscal_secundaria {
            Some(v) => v
                .to_string()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
            _ => vec![],
        };

        let cnaes_fiscais_secundarias = cnaes::table
            .filter(cnaes::id.eq_any(cnaes_fiscais_secundarias))
            .load::<CNAE>(&*connection)?;

        Ok(cnaes_fiscais_secundarias)
    }

    pub fn tipo_logradouro(&self) -> &Option<String> {
        &self.tipo_logradouro
    }

    pub fn logradouro(&self) -> &Option<String> {
        &self.logradouro
    }

    pub fn numero(&self) -> &Option<String> {
        &self.numero
    }

    pub fn complemento(&self) -> &Option<String> {
        &self.complemento
    }

    pub fn bairro(&self) -> &Option<String> {
        &self.bairro
    }

    pub fn cep(&self) -> &Option<String> {
        &self.cep
    }

    pub fn uf(&self) -> &Option<String> {
        &self.uf
    }

    pub fn municipio(&self, context: &Context) -> Result<Option<Municipio>, FieldError> {
        match self.municipio {
            Some(v) => {
                use data_models::schema::municipios;
                let connection = context.pool.clone().get()?;

                Ok(Some(
                    municipios::table
                        .filter(municipios::id.eq(v))
                        .first::<Municipio>(&*connection)?,
                ))
            }
            None => Ok(None),
        }
    }

    pub fn ddd1(&self) -> &Option<String> {
        &self.ddd1
    }

    pub fn telefone1(&self) -> &Option<String> {
        &self.telefone1
    }

    pub fn ddd2(&self) -> &Option<String> {
        &self.ddd2
    }

    pub fn telefone2(&self) -> &Option<String> {
        &self.telefone2
    }

    pub fn ddd_fax(&self) -> &Option<String> {
        &self.ddd_fax
    }

    pub fn telefone_fax(&self) -> &Option<String> {
        &self.telefone_fax
    }

    pub fn correio_eletronico(&self) -> &Option<String> {
        &self.correio_eletronico
    }

    pub fn situacao_especial(&self) -> &Option<String> {
        &self.situacao_especial
    }

    pub fn data_situacao_especial(&self) -> &Option<NaiveDate> {
        &self.data_situacao_especial
    }

    pub fn empresa(&self, context: &Context) -> Result<Empresa, FieldError> {
        use data_models::schema::empresas;
        let connection = context.pool.clone().get()?;

        Ok(empresas::table
            .filter(empresas::cnpj_basico.eq(&self.cnpj_basico))
            .first::<Empresa>(&*connection)?)
    }
}

#[derive(GraphQLEnum)]
#[graphql(description = "Situação Cadastral do Estabelecimento")]
enum SituacaoCadastral {
    NULA,
    ATIVA,
    SUSPENSA,
    INAPTA,
    BAIXADA,
    DESCONHECIDA,
}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "0.1"
    }

    fn cnae(context: &Context, id: String) -> Result<CNAE, FieldError> {
        use data_models::schema::cnaes;
        let connection = context.pool.clone().get()?;

        Ok(cnaes::table
            .filter(cnaes::id.eq(id.parse::<u32>()?))
            .first::<CNAE>(&*connection)?)
    }

    fn cnaes(context: &Context) -> Result<Vec<CNAE>, FieldError> {
        use data_models::schema::cnaes::dsl::*;

        let connection = context.pool.clone().get()?;

        Ok(cnaes.load::<CNAE>(&*connection)?)
    }

    fn natureza_juridica(context: &Context, id: String) -> Result<NaturezaJuridica, FieldError> {
        use data_models::schema::naturezas_juridicas;
        let connection = context.pool.clone().get()?;

        Ok(naturezas_juridicas::table
            .filter(naturezas_juridicas::id.eq(id.parse::<u16>()?))
            .first::<NaturezaJuridica>(&*connection)?)
    }

    fn naturezas_juridicas(context: &Context) -> Result<Vec<NaturezaJuridica>, FieldError> {
        use data_models::schema::naturezas_juridicas::dsl::*;
        let connection = context.pool.clone().get()?;

        Ok(naturezas_juridicas.load::<NaturezaJuridica>(&*connection)?)
    }

    fn municipio(context: &Context, id: String) -> Result<Municipio, FieldError> {
        use data_models::schema::municipios;
        let connection = context.pool.clone().get()?;

        Ok(municipios::table
            .filter(municipios::id.eq(id.parse::<u16>()?))
            .first::<Municipio>(&*connection)?)
    }

    fn municipios(context: &Context) -> Result<Vec<Municipio>, FieldError> {
        use data_models::schema::municipios::dsl::*;

        let connection = context.pool.clone().get()?;

        Ok(municipios.load::<Municipio>(&*connection)?)
    }

    fn pais(context: &Context, id: String) -> Result<Pais, FieldError> {
        use data_models::schema::paises;
        let connection = context.pool.clone().get()?;

        Ok(paises::table
            .filter(paises::id.eq(id.parse::<u16>()?))
            .first::<Pais>(&*connection)?)
    }

    fn paises(context: &Context) -> Result<Vec<Pais>, FieldError> {
        use data_models::schema::paises::dsl::*;

        let connection = context.pool.clone().get()?;

        Ok(paises.load::<Pais>(&*connection)?)
    }

    fn qualificacao_de_socio(
        context: &Context,
        id: String,
    ) -> Result<QualificacaoDeSocio, FieldError> {
        use data_models::schema::qualificacoes_de_socios;
        let connection = context.pool.clone().get()?;

        Ok(qualificacoes_de_socios::table
            .filter(qualificacoes_de_socios::id.eq(id.parse::<u8>()?))
            .first::<QualificacaoDeSocio>(&*connection)?)
    }

    fn qualificacoes_de_socios(context: &Context) -> Result<Vec<QualificacaoDeSocio>, FieldError> {
        use data_models::schema::qualificacoes_de_socios::dsl::*;

        let connection = context.pool.clone().get()?;

        Ok(qualificacoes_de_socios.load::<QualificacaoDeSocio>(&*connection)?)
    }

    fn motivo_de_situacao_cadastral(
        context: &Context,
        id: String,
    ) -> Result<MotivoDeSituacaoCadastral, FieldError> {
        use data_models::schema::motivos_de_situacoes_cadastrais;
        let connection = context.pool.clone().get()?;

        Ok(motivos_de_situacoes_cadastrais::table
            .filter(motivos_de_situacoes_cadastrais::id.eq(id.parse::<u8>()?))
            .first::<MotivoDeSituacaoCadastral>(&*connection)?)
    }

    fn motivos_de_situacoes_cadastrais(
        context: &Context,
    ) -> Result<Vec<MotivoDeSituacaoCadastral>, FieldError> {
        use data_models::schema::motivos_de_situacoes_cadastrais::dsl::*;
        let connection = context.pool.clone().get()?;

        Ok(motivos_de_situacoes_cadastrais.load::<MotivoDeSituacaoCadastral>(&*connection)?)
    }

    fn empresa(context: &Context, cnpj_basico: String) -> Result<Empresa, FieldError> {
        use data_models::schema::empresas;
        let connection = context.pool.clone().get()?;

        Ok(empresas::table
            .filter(empresas::cnpj_basico.eq(cnpj_basico))
            .first::<Empresa>(&*connection)?)
    }

    fn estabelecimento(
        context: &Context,
        cnpj_completo: String,
    ) -> Result<Estabelecimento, FieldError> {
        use data_models::schema::estabelecimentos;
        let connection = context.pool.clone().get()?;

        Ok(estabelecimentos::table
            .filter(estabelecimentos::cnpj_basico.eq(&cnpj_completo[..8]))
            .filter(estabelecimentos::cnpj_ordem.eq(&cnpj_completo[8..12]))
            .filter(estabelecimentos::cnpj_dv.eq(&cnpj_completo[12..]))
            .first::<Estabelecimento>(&*connection)?)
    }

    fn socios(context: &Context, cnpj_basico: String) -> Result<Vec<Socio>, FieldError> {
        use data_models::schema::socios;
        let connection = context.pool.clone().get()?;

        Ok(socios::table
            .filter(socios::cnpj_basico.eq(cnpj_basico))
            .load::<Socio>(&*connection)?)
    }

    fn simples(context: &Context, cnpj_basico: String) -> Result<Simples, FieldError> {
        use data_models::schema::simples;
        let connection = context.pool.clone().get()?;

        Ok(simples::table
            .filter(simples::cnpj_basico.eq(cnpj_basico))
            .first::<Simples>(&*connection)?)
    }    
    
}

pub struct Context {
    pub pool: r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
