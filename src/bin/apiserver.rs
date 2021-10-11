use std::env;

use dotenv::dotenv;

#[macro_use] 
extern crate rocket;

use dados_abertos_cnpj::{error::CustomError, models::{CNAE, Empresa, Estabelecimento, MotivoDeSituacaoCadastral, Municipio, NaturezaJuridica, Pais, QualificacaoDeSocio}};
use dados_abertos_cnpj::schema::{empresas, estabelecimentos, cnaes, paises, municipios, naturezas_juridicas, motivos_de_situacoes_cadastrais, qualificacoes_de_socios};
use diesel::prelude::*;
use rocket::{figment::{map, value::{Map, Value}}, serde::{Serialize, json::Json}};

use rocket_sync_db_pools::database;

#[database("cnpj_db")]
struct DBPool(diesel::MysqlConnection);

#[derive(Serialize)]
struct CnaeResult {
    cnae: CNAE,
}

#[get("/cnaes/<cnae_num>", format = "json")] 
async fn get_cnaes(conn: DBPool, cnae_num: u32) -> Result<Json<CnaeResult>, CustomError> {

    let cnae = conn
        .run(move |c| {
            cnaes::table
                .filter(cnaes::id.eq(cnae_num))
                .first::<CNAE>(c)
        })
        .await?;
    
    Ok(
        Json(
            CnaeResult {
                cnae,
            }
        )
    )
}

#[derive(Serialize)]
struct EmpresaResult {
    empresa: Empresa,
    natureza_juridica: Option<NaturezaJuridica>,
    qualificacao_do_responsavel: Option<QualificacaoDeSocio>,
}

#[get("/empresas/<cnpjbas>", format = "json")] // cnpjbas = CNPJ Básico, ou seja, os 8 primeiros digitos do CNPJ
async fn get_empresas(conn: DBPool, cnpjbas: String) -> Result<Json<EmpresaResult>, CustomError> {

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
    cnaes_fiscais_secundarias: Option<Vec<CNAE>>,
    empresa: Option<Empresa>,
    natureza_juridica: Option<NaturezaJuridica>,
    qualificacao_do_responsavel: Option<QualificacaoDeSocio>,
}

#[get("/estabelecimentos/<com_cnaes_secundarias>/<cnpj_completo>", format = "json")]
async fn get_estabelecimentos(conn: DBPool, cnpj_completo: String, com_cnaes_secundarias: bool) -> Result<Json<EstabelecimentoResult>, CustomError> {

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

    if com_cnaes_secundarias {
            let cnaes_fiscais_secundarias: Vec<u32> = match &estabelecimento.cnae_fiscal_secundaria {
                Some(v) => v.to_string().split(",").map(|s| s.parse().unwrap()).collect(),
                _ => vec![]
            };
           
            let cnaes_fiscais_secundarias = conn
                .run(|c| {
                    cnaes::table
                        .filter(cnaes::id.eq_any(cnaes_fiscais_secundarias))
                        .load::<CNAE>(c)
                })
                .await?;
        
            Ok(Json(EstabelecimentoResult{
                estabelecimento,
                motivo_situacao_cadastral,
                pais,
                municipio,
                cnae_fiscal_principal,
                cnaes_fiscais_secundarias: Some(cnaes_fiscais_secundarias),
                empresa,
                natureza_juridica,
                qualificacao_do_responsavel,
            }))            

        } else {
            Ok(Json(EstabelecimentoResult{
                estabelecimento,
                motivo_situacao_cadastral,
                pais,
                municipio,
                cnae_fiscal_principal,
                cnaes_fiscais_secundarias: None,
                empresa,
                natureza_juridica,
                qualificacao_do_responsavel,
            }))
        }
    
}

#[launch]
fn rocket() -> _ {

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment().merge(("databases", map!["cnpj_db" => db]));

    rocket::custom(figment)
        .mount("/api", routes![
            get_cnaes,
            get_empresas,
            get_estabelecimentos
        ])
        .attach(DBPool::fairing())
}