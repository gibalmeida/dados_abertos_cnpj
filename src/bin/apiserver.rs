#[macro_use] 
extern crate rocket;

use dados_abertos_cnpj::{error::CustomError, models::{CNAE, Empresa, Estabelecimento, MotivoDeSituacaoCadastral, Municipio, NaturezaJuridica, Pais, QualificacaoDeSocio}};
use dados_abertos_cnpj::schema::{empresas, estabelecimentos, cnaes, paises, municipios, naturezas_juridicas, motivos_de_situacoes_cadastrais, qualificacoes_de_socios};
use diesel::prelude::*;
use rocket::serde::{Serialize, json::Json};

use rocket_sync_db_pools::database;

#[database("cnpj_db")]
struct CnpjDbConn(diesel::MysqlConnection);


#[derive(Serialize)]
struct EmpresaResult {
    empresa: Empresa,
    natureza_juridica: Option<NaturezaJuridica>,
    qualificacao_do_responsavel: Option<QualificacaoDeSocio>,
}

#[get("/empresas/<cnpjbas>", format = "json")] // cnpjbas = CNPJ Básico, ou seja, os 8 primeiros digitos do CNPJ
async fn get_empresas(conn: CnpjDbConn, cnpjbas: String) -> Result<Json<EmpresaResult>, CustomError> {

    let query_result = conn
        .run(move |c| {
            empresas::table
                .filter(empresas::cnpj_basico.eq(cnpjbas))
                .left_join(naturezas_juridicas::table)
                .left_join(qualificacoes_de_socios::table)
                .first::<(Empresa,Option<NaturezaJuridica>,Option<QualificacaoDeSocio>)>(c)
        })
        .await?;
    
    let (
        empresa,
        natureza_juridica,
        qualificacao_do_responsavel
    ) = query_result;

    Ok(
        Json(
            EmpresaResult {
                empresa,
                natureza_juridica,
                qualificacao_do_responsavel,
            }
        )
    )
}

#[derive(Serialize)]
struct EstabelecimentoResult {
    estabelecimento: Estabelecimento,
    motivo_situacao_cadastral: Option<MotivoDeSituacaoCadastral>,
    pais: Option<Pais>,
    municipio: Option<Municipio>,
    cnae_fiscal_principal: Option<CNAE>,
    // cnae_fiscal_secundaria: Vec<CNAE>,
    cnaes_fiscais_secundarias: Vec<i32>,
    empresa: Option<Empresa>,
    natureza_juridica: Option<NaturezaJuridica>,
    qualificacao_do_responsavel: Option<QualificacaoDeSocio>,
}

#[get("/estabelecimentos/<cnpj_completo>", format = "json")]
async fn get_estabelecimentos(conn: CnpjDbConn, cnpj_completo: String) -> Result<Json<EstabelecimentoResult>, CustomError> {

    let query_result = conn
        .run(move |c| {
            estabelecimentos::table
                .filter(estabelecimentos::cnpj_basico.eq(&cnpj_completo[..8]))
                .filter(estabelecimentos::cnpj_ordem.eq(&cnpj_completo[8..12]))
                .filter(estabelecimentos::cnpj_dv.eq(&cnpj_completo[12..]))
                .left_join(motivos_de_situacoes_cadastrais::table)
                .left_join(paises::table)
                .left_join(municipios::table)
                .left_join(cnaes::table)
                .left_join(empresas::table)
                .left_join(naturezas_juridicas::table.on(empresas::natureza_juridica.eq(naturezas_juridicas::id.nullable())))
                .left_join(qualificacoes_de_socios::table.on(empresas::qualificacao_do_responsavel.eq(qualificacoes_de_socios::id.nullable())))
                .first::<(
                    Estabelecimento,
                    Option<MotivoDeSituacaoCadastral>,
                    Option<Pais>,
                    Option<Municipio>,
                    Option<CNAE>,
                    Option<Empresa>,
                    Option<NaturezaJuridica>,
                    Option<QualificacaoDeSocio>,
                )>(c)
        })
        .await?;
        
    let (
        estabelecimento, 
        motivo_situacao_cadastral, 
        pais,
        municipio,
        cnae_fiscal_principal,
        empresa, 
        natureza_juridica,
        qualificacao_do_responsavel,
    ) = query_result;

    let cnaes_fiscais_secundarias: Vec<i32> = match &estabelecimento.cnae_fiscal_secundaria {
        Some(v) => v.to_string().split(",").map(|s| s.parse().unwrap()).collect(),
        _ => vec![]
    };

    //let cnaes_secundarios = vec!["aaa","bbb"];
    
    // let result_query2 = conn
    //     .run(|c| {
    //         cnaes::table
    //             .filter(cnaes::id.eq(cnaes_secundarios))
    //             .load::<Vec<CNAE>>(c)
    //     })
    //     .await?;

    Ok(Json(EstabelecimentoResult{
        estabelecimento,
        motivo_situacao_cadastral,
        pais,
        municipio,
        cnae_fiscal_principal,
        cnaes_fiscais_secundarias,
        empresa,
        natureza_juridica,
        qualificacao_do_responsavel,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CnpjDbConn::fairing())
        .mount("/api", routes![get_empresas,get_estabelecimentos])
}