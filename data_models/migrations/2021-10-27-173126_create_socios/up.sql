CREATE TABLE socios (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    cnpj_basico CHAR(8) NOT NULL,
    identificador_de_socio TINYINT UNSIGNED NOT NULL,
    nome_ou_razao_social_do_socio VARCHAR(150) NOT NULL,
    cnpj_ou_cpf_do_socio CHAR(14),
    qualificacao_do_socio TINYINT UNSIGNED NOT NULL,
    data_de_entrada_na_sociedade DATE NOT NULL,
    pais_do_socio SMALLINT UNSIGNED,
    cpf_do_representante_legal CHAR(11) NOT NULL,
    nome_do_representante_legal VARCHAR(60) NOT NULL,
    qualificacao_do_representante_legal TINYINT UNSIGNED NOT NULL,
    faixa_etaria_do_socio TINYINT UNSIGNED NOT NULL
)