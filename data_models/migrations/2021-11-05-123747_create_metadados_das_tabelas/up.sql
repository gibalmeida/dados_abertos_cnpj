CREATE TABLE metadados_das_tabelas (
    tabela VARCHAR(50) NOT NULL PRIMARY KEY,
    data_hora_de_atualizacao TIMESTAMP NOT NULL,
    data_hora_de_importacao TIMESTAMP NOT NULL
)