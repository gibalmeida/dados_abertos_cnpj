CREATE TABLE estabelecimentos (
    cnpj_basico CHAR(8),
    cnpj_ordem CHAR(4),
    cnpj_dv CHAR(2) NOT NULL,
    identificador_matriz_filial CHAR(1) NOT NULL,
    nome_fantasia VARCHAR(200),
    situacao_cadastral CHAR(2) NOT NULL,
    data_situacao_cadastral DATE,
    motivo_situacao_cadastral CHAR(10),
    nome_da_cidade_no_exterior VARCHAR(200),
    pais INT,
    data_de_inicio_da_atividade DATE,
    cnae_fiscal_principal INT,
    cnae_fiscal_secundaria INT,
    tipo_logradouro VARCHAR(50),
    logradouro VARCHAR(300),
    numero VARCHAR(10),
    complemento VARCHAR(200),
    bairro VARCHAR(200),
    cep VARCHAR(10),
    uf CHAR(2),
    municipio INT,
    ddd1 INT,
    telefone1 BIGINT,
    ddd2 INT,
    telefone2 BIGINT,
    ddd_fax INT,
    telefone_fax BIGINT,
    correio_eletronico VARCHAR(200),
    situacao_especial VARCHAR(100),
    data_situacao_especial DATE,
    PRIMARY KEY (cnpj_basico, cnpj_ordem, cnpj_dv)
)