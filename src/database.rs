use std::env;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use crate::models::*;

pub struct Database {
    db_connection: MysqlConnection
}

impl Database {
    pub fn new() -> Database {
        let db_connection = Self::establish_connection();

        Database {
            db_connection
        }
    }

    pub fn upsert_empresa(&self, new_empresa: &NewEmpresa ) -> QueryResult<usize> {
        use crate::schema::empresas;
    
        diesel::replace_into(empresas::table)
            .values(new_empresa)
            .execute(&self.db_connection)
    }

    pub fn upsert_estabelecimento(&self, new_estabelecimento: &NewEstabelecimento ) -> QueryResult<usize> {
        use crate::schema::estabelecimentos;
    
        diesel::replace_into(estabelecimentos::table)
            .values(new_estabelecimento)
            .execute(&self.db_connection)
    }

    pub fn upsert_cnae(&self, new_cnae: &NewCNAE) -> QueryResult<usize> {
        use crate::schema::cnaes;

        diesel::replace_into(cnaes::table)
            .values(new_cnae)
            .execute(&self.db_connection)
    }

    pub fn upsert_natureza_juridica(&self, new_natureza_juridica: &NewNaturezaJuridica) -> QueryResult<usize> {
        use crate::schema::naturezas_juridicas;
    
        diesel::replace_into(naturezas_juridicas::table)
            .values(new_natureza_juridica)
            .execute(&self.db_connection)
    }

    pub fn upsert_paises(&self, new_pais: &NewPais) -> QueryResult<usize> {
        use crate::schema::paises;
    
        diesel::replace_into(paises::table)
            .values(new_pais)
            .execute(&self.db_connection)
    }
    
    pub fn upsert_municipios(&self, new_municipio: &NewMunicipio) -> QueryResult<usize> {
        use crate::schema::municipios;
    
        diesel::replace_into(municipios::table)
            .values(new_municipio)
            .execute(&self.db_connection)
    }
    
    pub fn upsert_qualificacoes_de_socios(&self, new_qualif_socio: &NewQualificacaoDeSocio) -> QueryResult<usize> {
        use crate::schema::qualificacoes_de_socios;
    
        diesel::replace_into(qualificacoes_de_socios::table)
            .values(new_qualif_socio)
            .execute(&self.db_connection)
    }

    pub fn upsert_motivo_de_situacao_cadastral(&self, new_motivo_sit_cad: &NewMotivoDeSituacaoCadastral) -> QueryResult<usize> {
        use crate::schema::motivos_de_situacoes_cadastrais;
    
        diesel::replace_into(motivos_de_situacoes_cadastrais::table)
            .values(new_motivo_sit_cad)
            .execute(&self.db_connection)
    }

    fn establish_connection() -> MysqlConnection {
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL precisa ser definida!");
        MysqlConnection::establish(&database_url)
            .expect(&format!("Erro ao conectar em {}", database_url))
    }
            
}
