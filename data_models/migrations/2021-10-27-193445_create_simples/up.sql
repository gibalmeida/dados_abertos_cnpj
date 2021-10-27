CREATE TABLE simples (
    cnpj_basico CHAR(8) NOT NULL PRIMARY KEY,
    opcao_pelo_simples CHAR(1) NOT NULL,
    data_de_opcao_pelo_simples DATE,
    data_de_exclusao_do_simples DATE,
    opcao_pelo_mei CHAR(1) NOT NULL,
    data_de_opcao_pelo_mei DATE,
    data_de_exclusao_do_mei DATE
)