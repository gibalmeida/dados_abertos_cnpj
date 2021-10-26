use std::env;

use data_models::models::*;
use diesel::mysql::MysqlConnection;
use diesel::{prelude::*, sql_query};
use dotenv::dotenv;

use crate::config::Config;

pub struct Database<'a> {
    db_connection: MysqlConnection,
    config: &'a Config<'a>,
}

impl<'a> Database<'a> {
    pub fn new(config: &'a Config) -> Database<'a> {
        let db_connection = Self::establish_connection();        

        Database { db_connection, config }
    }

    pub fn before_table_update(&self) {

        let table_name = self.config.tipo_de_arquivo().table_name();

        // Desativa o autocommit, checagens de chaves estrangeiras e checagens de chaves únicas (unique)
        // para dar mais performance nas gravações do banco de dados (vide: https://dev.mysql.com/doc/refman/8.0/en/optimizing-innodb-bulk-data-loading.html)
        sql_query("SET autocommit=0")
            .execute(&self.db_connection)
            .expect("Erro ao desativar o autocommit!");
        sql_query("SET foreign_key_checks = 0")
            .execute(&self.db_connection)
            .expect("Erro ao desativar checagens de chaves estrangeiras!");
        sql_query("SET unique_checks=0")
            .execute(&self.db_connection)
            .expect("Erro ao desativar checagens de chaves únicas!");        
            
        if self.config.truncate_table() && self.config.is_first_file_number() {
            self.truncate_table(table_name);
        }
        
        if self.config.disable_keys() { 
            self.disable_keys(table_name);
        }

    }

    pub fn after_table_update(&self) {

        let table_name = self.config.tipo_de_arquivo().table_name();
        
        if self.config.disable_keys() && self.config.is_last_file_number() { 
            self.enable_keys(table_name);
        }

        self.commit();

    }

    pub fn commit(&self) {
        // Faz o COMMIT para gravar os registros no banco de dados
        sql_query("COMMIT")
            .execute(&self.db_connection)
            .expect("Erro ao executar o commit!");
        sql_query("SET foreign_key_checks = 1")
            .execute(&self.db_connection)
            .expect("Erro ao ativar novamente checagens de chaves estrangeiras!");
        sql_query("SET unique_checks=1")
            .execute(&self.db_connection)
            .expect("Erro ao ativar novamente checagens de chaves únicas!");
    }

    pub fn enable_keys(&self, table_name: &str) {
        sql_query(format!("ALTER TABLE {} ENABLE KEYS", table_name))
            .execute(&self.db_connection)
            .expect("Erro ao habilitar chaves");
    }

    pub fn disable_keys(&self, table_name: &str) {
        sql_query(format!("ALTER TABLE {} DISABLE KEYS", table_name))
            .execute(&self.db_connection)
            .expect("Erro ao habilitar chaves");
    }

    // pub fn drop_foreign_keys(&self) {

    // sql_query("ALTER TABLE estabelecimentos
    // DROP FOREIGN KEY FK_EstabSitCad,
    // DROP FOREIGN KEY FK_EstabMotivCad,
    // DROP FOREIGN KEY FK_EstabPais,
    // DROP FOREIGN KEY FK_EstabCnaePrinc,
    // DROP FOREIGN KEY FK_EstabMunic;")
    //     .execute(&self.db_connection)
    //     .expect("Erro ao remover as chaves estrangeiras da tabela estabelecimentos");
    
    // sql_query("ALTER TABLE empresas
    // DROP FOREIGN KEY FK_EmpNatJur,
    // DROP FOREIGN KEY FK_EmpQualResp;")
    //     .execute(&self.db_connection)
    //     .expect("Erro ao remover as chaves estrangeiras da tabela empresas");    

    // }

    pub fn truncate_table(&self, table_name: &str) {
        sql_query(format!("TRUNCATE TABLE {}", table_name))
            .execute(&self.db_connection)
            .expect("Erro ao zerar a tabela.");
    }
    

    pub fn upsert_empresa(&self, new_empresa: &Vec<NewEmpresa>) -> QueryResult<usize> {
        use data_models::schema::empresas;

        if self.config.truncate_table() || self.config.empty() {
            // se a tabela foi zerada, então é melhor utilizar o insert ao invés do replace_into
            return diesel::insert_into(empresas::table)
                .values(new_empresa)
                .execute(&self.db_connection)
        }

        diesel::replace_into(empresas::table)
            .values(new_empresa)
            .execute(&self.db_connection)
    }

    pub fn upsert_estabelecimento(
        &self,
        new_estabelecimento: &Vec<NewEstabelecimento>,
    ) -> QueryResult<usize> {
        use data_models::schema::estabelecimentos;

        if self.config.truncate_table() || self.config.empty() {
            // se a tabela foi zerada, então é melhor utilizar o insert ao invés do replace_into
            return diesel::insert_into(estabelecimentos::table)
                .values(new_estabelecimento)
                .execute(&self.db_connection)
        }

        diesel::replace_into(estabelecimentos::table)
            .values(new_estabelecimento)
            .execute(&self.db_connection)
    }

    pub fn upsert_cnae(&self, new_cnae: &NewCNAE) -> QueryResult<usize> {
        use data_models::schema::cnaes;

        diesel::replace_into(cnaes::table)
            .values(new_cnae)
            .execute(&self.db_connection)
    }

    pub fn upsert_natureza_juridica(
        &self,
        new_natureza_juridica: &NewNaturezaJuridica,
    ) -> QueryResult<usize> {
        use data_models::schema::naturezas_juridicas;

        diesel::replace_into(naturezas_juridicas::table)
            .values(new_natureza_juridica)
            .execute(&self.db_connection)
    }

    pub fn upsert_paises(&self, new_pais: &NewPais) -> QueryResult<usize> {
        use data_models::schema::paises;

        diesel::replace_into(paises::table)
            .values(new_pais)
            .execute(&self.db_connection)
    }

    pub fn upsert_municipios(&self, new_municipio: &NewMunicipio) -> QueryResult<usize> {
        use data_models::schema::municipios;

        diesel::replace_into(municipios::table)
            .values(new_municipio)
            .execute(&self.db_connection)
    }

    pub fn upsert_qualificacoes_de_socios(
        &self,
        new_qualif_socio: &NewQualificacaoDeSocio,
    ) -> QueryResult<usize> {
        use data_models::schema::qualificacoes_de_socios;

        diesel::replace_into(qualificacoes_de_socios::table)
            .values(new_qualif_socio)
            .execute(&self.db_connection)
    }

    pub fn upsert_motivo_de_situacao_cadastral(
        &self,
        new_motivo_sit_cad: &NewMotivoDeSituacaoCadastral,
    ) -> QueryResult<usize> {
        use data_models::schema::motivos_de_situacoes_cadastrais;

        diesel::replace_into(motivos_de_situacoes_cadastrais::table)
            .values(new_motivo_sit_cad)
            .execute(&self.db_connection)
    }

    pub fn upsert_arquivo_importado(
        &self,
        new_arquivo_importado: &NewArquivoImportado,
    ) -> QueryResult<usize> {
        use data_models::schema::arquivos_importados;

        diesel::replace_into(arquivos_importados::table)
            .values(new_arquivo_importado)
            .execute(&self.db_connection)
    }    

    fn establish_connection() -> MysqlConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL precisa ser definida!");
        MysqlConnection::establish(&database_url)
            .expect(&format!("Erro ao conectar em {}", database_url))
    }
}
