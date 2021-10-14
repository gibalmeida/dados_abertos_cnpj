CREATE TABLE empresas (
    cnpj_basico CHAR(8) PRIMARY KEY,
    razao_social VARCHAR(250) NOT NULL,
    natureza_juridica INT,
    qualificacao_do_responsavel INT,
    capital_social DECIMAL(15,2),
    porte CHAR(2),
    ente_federativo_responsavel VARCHAR(100)
)