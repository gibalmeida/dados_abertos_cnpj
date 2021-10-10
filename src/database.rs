use std::env;

use diesel::{prelude::*, sql_query};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use crate::models::*;

pub struct Database {
    db_connection: MysqlConnection
}

impl Database {
    pub fn new() -> Database {
        let db_connection = Self::establish_connection();

        //Aumenta o tamanho máximo dos pacotes para 1Gb (Não dá para ir além de 1Gb, pois o MySQL não aceita!)
        sql_query("SET GLOBAL max_allowed_packet=1073741824").execute(&db_connection).expect("Erro ao executar ao aumentar o tamanho máximo do pacote.");

        // Desativa o autocommit, checagens de chaves estrangeiras e checagens de chaves únicas (unique)
        // para dar mais performance nas gravações do banco de dados (vide: https://dev.mysql.com/doc/refman/8.0/en/optimizing-innodb-bulk-data-loading.html)
        sql_query("SET autocommit=0").execute(&db_connection).expect("Erro ao desativar o autocommit!");
        sql_query("SET foreign_key_checks = 0").execute(&db_connection).expect("Erro ao desativar checagens de chaves estrangeiras!");
        sql_query("SET unique_checks=0").execute(&db_connection).expect("Erro ao desativar checagens de chaves únicas!");
        

        Database {
            db_connection
        }
    }

    pub fn commit(&self) {
        // Faz o COMMIT para gravar os registros no banco de dados
        sql_query("COMMIT").execute(&self.db_connection).expect("Erro ao executar o commit!");
        sql_query("SET foreign_key_checks = 1").execute(&self.db_connection).expect("Erro ao ativar novamente checagens de chaves estrangeiras!");
        sql_query("SET unique_checks=1").execute(&self.db_connection).expect("Erro ao ativar novamente checagens de chaves únicas!");        
    }

    pub fn insert_empresa(&self, new_empresa: &Vec<NewEmpresa> ) -> QueryResult<usize> {
        use crate::schema::empresas;
    
        diesel::insert_into(empresas::table)
            .values(new_empresa)
            .execute(&self.db_connection)
    }

    pub fn insert_estabelecimento(&self, new_estabelecimento: &Vec<NewEstabelecimento> ) -> QueryResult<usize> {
        use crate::schema::estabelecimentos;
    
        diesel::insert_into(estabelecimentos::table)
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
