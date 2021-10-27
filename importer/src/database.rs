use std::env;

use data_models::models::*;
use diesel::mysql::MysqlConnection;
use diesel::{prelude::*, sql_query};
use dotenv::dotenv;

use crate::config::Config;
use crate::tipo_de_arquivo::TipoDeArquivo;

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
        let queries = vec![
            "SET autocommit=0", // Desabilita o Auto Commit (a gravação efetiva dos registros irão ocorrer no final do processamento do arquivo)
            "SET foreign_key_checks = 0", // Desativa a checagem de chaves estrageiras de modo a aumentar a performance de população das tabelas
            "SET unique_checks=0", // Desativa a checagem de chaves únicas, também para aumentar a performance.
        ];
        self.sql_queries(queries, true); 
            
        if self.config.truncate_table() && self.config.is_first_file_number() {
            self.truncate_table(table_name);
        }

        if self.config.is_first_file_number() && self.config.drop_indexes() {
            self.drop_indexes_and_primary_keys();
        }
        
        // if self.config.disable_keys() { 
        //     self.disable_keys(table_name);
        // }

    }

    pub fn after_table_update(&self) {

        // let table_name = self.config.tipo_de_arquivo().table_name();
        // 
        // if self.config.disable_keys() && self.config.is_last_file_number() { 
        //     self.enable_keys(table_name);
        // }

        if self.config.is_last_file_number() && self.config.drop_indexes() {
            self.add_indexes_and_primary_keys();
        }        

        self.commit();

        // Habilitando novamente algumas coisas que foram desabilitadas antes de iniciar o processamento do arquivo
        let queries = vec![
            "SET foreign_key_checks = 1", // Habilita a checagem de chaves estrangeiras
            "SET unique_checks=1", // Habilita a checagem de chaves únicas
            "SET autocommit=1", // Habilita o Auto Commit
        ];

        self.sql_queries(queries, true);

    }

    pub fn commit(&self) {
        // Faz o COMMIT para gravar os registros no banco de dados
        sql_query("COMMIT")
            .execute(&self.db_connection)
            .expect("Erro ao executar o commit!");

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

    pub fn drop_indexes_and_primary_keys(&self) {


        let table_name = self.config.tipo_de_arquivo().table_name();

        let alter_table_modifications = match self.config.tipo_de_arquivo() {
            TipoDeArquivo::Estabelecimentos => {

                vec![
                    (table_name,"DROP FOREIGN KEY FK_EstabSitCad"),
                    (table_name,"DROP INDEX FK_EstabSitCad"),
                    (table_name,"DROP FOREIGN KEY FK_EstabMotivCad"),
                    (table_name,"DROP INDEX FK_EstabMotivCad"),
                    (table_name,"DROP FOREIGN KEY FK_EstabPais"),
                    (table_name,"DROP INDEX FK_EstabPais"),
                    (table_name,"DROP FOREIGN KEY FK_EstabCnaePrinc"),
                    (table_name,"DROP INDEX FK_EstabCnaePrinc"),
                    (table_name,"DROP FOREIGN KEY FK_EstabMunic"),
                    (table_name,"DROP INDEX FK_EstabMunic"),
                    (table_name,"DROP FOREIGN KEY FK_EstabEmp"),
                    (table_name,"DROP PRIMARY KEY"),
                ]
                
            },
            TipoDeArquivo::Empresas => {

                vec![
                    (table_name,"DROP FOREIGN KEY FK_EmpNatJur"),
                    (table_name,"DROP INDEX FK_EmpNatJur"),
                    (table_name,"DROP FOREIGN KEY FK_EmpQualResp"),
                    (table_name,"DROP INDEX FK_EmpQualResp"),
                    (TipoDeArquivo::Estabelecimentos.table_name(),"DROP FOREIGN KEY FK_EstabEmp"),
                    (table_name,"DROP PRIMARY KEY"),
                ]                
        
            },
            _ => {
                println!("Aviso! Não há necessidade de remover os índices da tabela {} pois a quantidade de registros é muito pequena e não haveria nenhum ganho de performance.", table_name);
                return ();
            }
        };
        self.alter_table_modifications( alter_table_modifications, false)
    }

    pub fn add_indexes_and_primary_keys(&self) {

        let table_name = self.config.tipo_de_arquivo().table_name();

        let alter_table_modifications = match self.config.tipo_de_arquivo() {
            TipoDeArquivo::Estabelecimentos => {
                vec![
                    (table_name,"ADD PRIMARY KEY (cnpj_basico, cnpj_ordem, cnpj_dv)"),
                    (table_name,"ADD CONSTRAINT FK_EstabEmp FOREIGN KEY (cnpj_basico) REFERENCES empresas(cnpj_basico)"),
                    (table_name,"ADD CONSTRAINT FK_EstabMotivCad FOREIGN KEY (motivo_situacao_cadastral) REFERENCES motivos_de_situacoes_cadastrais(id)"),
                    (table_name,"ADD CONSTRAINT FK_EstabPais FOREIGN KEY (pais) REFERENCES paises(id)"),
                    (table_name,"ADD CONSTRAINT FK_EstabCnaePrinc FOREIGN KEY (cnae_fiscal_principal) REFERENCES cnaes(id)"),
                    (table_name,"ADD CONSTRAINT FK_EstabMunic FOREIGN KEY (municipio) REFERENCES municipios(id)"),
                    (table_name,"ADD CONSTRAINT FK_EstabSitCad FOREIGN KEY (situacao_cadastral) REFERENCES situacoes_cadastrais(id)")
                ]                
            },
            TipoDeArquivo::Empresas => {

                vec![
                    (table_name,"ADD PRIMARY KEY (cnpj_basico)"),
                    (table_name,"ADD CONSTRAINT FK_EmpNatJur FOREIGN KEY (natureza_juridica) REFERENCES naturezas_juridicas(id)"),
                    (table_name,"ADD CONSTRAINT FK_EmpQualResp FOREIGN KEY (qualificacao_do_responsavel) REFERENCES qualificacoes_de_socios(id)"),
                    (TipoDeArquivo::Estabelecimentos.table_name(),"ADD CONSTRAINT FK_EstabEmp FOREIGN KEY (cnpj_basico) REFERENCES empresas(cnpj_basico)"),
                ]     
            },
            _ => {
                println!("Aviso! Não há necessidade de remover os índices da tabela {} pois a quantidade de registros é muito pequena e não haveria nenhum ganho de performance.", self.config.tipo_de_arquivo().table_name());
                return;
            }
        };
        self.alter_table_modifications(alter_table_modifications, false);
    }    

    pub fn truncate_table(&self, table_name: &str) {
        sql_query(format!("TRUNCATE TABLE {}", table_name))
            .execute(&self.db_connection)
            .expect("Erro ao zerar a tabela.");
    }
    
    pub fn alter_table_modifications(&self, alter_table_lines: Vec<(&str,&str)>, panic: bool ) {
        for (table_name, line) in alter_table_lines {
            let query = format!("ALTER TABLE {} {}",table_name, line);
            sql_query(&query)
                .execute(&self.db_connection)
                .unwrap_or_else(|error| {
                    if panic {
                        panic!("Erro fatal! O seguinte erro ocorreu ao executar a query {{{}}}': {:?}", query, error);
                    }
                    println!("Aviso! O seguinte erro ocorreu ao executar a query {{{}}}': {:?}", query, error);
                    0
                });
        }
    }

    pub fn sql_queries(&self, queries: Vec<&str>, panic: bool) {

        for query in queries {
            sql_query(query)
            .execute(&self.db_connection)
            .unwrap_or_else(|error| {
                if panic {
                    panic!("Erro fatal! O seguinte erro ocorreu ao executar a query {{{}}}': {:?}", query, error);
                }
                println!("Aviso! O seguinte erro ocorreu ao executar a query {{{}}}': {:?}", query, error);
                0
            });    

        }
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
