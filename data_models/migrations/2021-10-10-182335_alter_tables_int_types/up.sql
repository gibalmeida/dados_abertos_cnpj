ALTER TABLE estabelecimentos
    DROP FOREIGN KEY FK_EstabSitCad,
    DROP FOREIGN KEY FK_EstabMotivCad,
    DROP FOREIGN KEY FK_EstabPais,
    DROP FOREIGN KEY FK_EstabCnaePrinc,
    DROP FOREIGN KEY FK_EstabMunic;
ALTER TABLE empresas
    DROP FOREIGN KEY FK_EmpNatJur,
    DROP FOREIGN KEY FK_EmpQualResp;
ALTER TABLE motivos_de_situacoes_cadastrais
    MODIFY COLUMN id TINYINT UNSIGNED;
ALTER TABLE situacoes_cadastrais
    MODIFY COLUMN id TINYINT UNSIGNED;
ALTER TABLE naturezas_juridicas
    MODIFY COLUMN id SMALLINT UNSIGNED;
ALTER TABLE qualificacoes_de_socios
    MODIFY COLUMN id TINYINT UNSIGNED;
ALTER TABLE paises
    MODIFY COLUMN id SMALLINT UNSIGNED;
ALTER TABLE municipios
    MODIFY COLUMN id SMALLINT UNSIGNED;
ALTER TABLE cnaes
    MODIFY COLUMN id INT UNSIGNED;
ALTER TABLE empresas 
    MODIFY COLUMN natureza_juridica SMALLINT UNSIGNED,
    MODIFY COLUMN qualificacao_do_responsavel TINYINT UNSIGNED;
ALTER TABLE estabelecimentos
    MODIFY COLUMN situacao_cadastral TINYINT UNSIGNED NOT NULL,
    MODIFY COLUMN motivo_situacao_cadastral TINYINT UNSIGNED,
    MODIFY COLUMN cnae_fiscal_principal INT UNSIGNED,
    MODIFY COLUMN pais SMALLINT UNSIGNED,
    MODIFY COLUMN municipio SMALLINT UNSIGNED,
    MODIFY COLUMN ddd1 VARCHAR(5),
    MODIFY COLUMN telefone1 VARCHAR(10),
    MODIFY COLUMN ddd2 VARCHAR(5),
    MODIFY COLUMN telefone2 VARCHAR(10),
    MODIFY COLUMN ddd_fax VARCHAR(5),
    MODIFY COLUMN telefone_fax VARCHAR(10);
ALTER TABLE estabelecimentos
    ADD CONSTRAINT FK_EstabSitCad FOREIGN KEY (situacao_cadastral) REFERENCES situacoes_cadastrais(id), 
    ADD CONSTRAINT FK_EstabMotivCad FOREIGN KEY (motivo_situacao_cadastral) REFERENCES motivos_de_situacoes_cadastrais(id),
    ADD CONSTRAINT FK_EstabPais FOREIGN KEY (pais) REFERENCES paises(id),
    ADD CONSTRAINT FK_EstabCnaePrinc FOREIGN KEY (cnae_fiscal_principal) REFERENCES cnaes(id),
    ADD CONSTRAINT FK_EstabMunic FOREIGN KEY (municipio) REFERENCES municipios(id);
ALTER TABLE empresas
    ADD CONSTRAINT FK_EmpNatJur FOREIGN KEY (natureza_juridica) REFERENCES naturezas_juridicas(id),
    ADD CONSTRAINT FK_EmpQualResp FOREIGN KEY (qualificacao_do_responsavel) REFERENCES qualificacoes_de_socios(id);