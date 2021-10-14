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
    MODIFY COLUMN id INT;
ALTER TABLE situacoes_cadastrais
    MODIFY COLUMN id INT;
ALTER TABLE naturezas_juridicas
    MODIFY COLUMN id INT;
ALTER TABLE qualificacoes_de_socios
    MODIFY COLUMN id INT;
ALTER TABLE paises
    MODIFY COLUMN id INT;
ALTER TABLE municipios
    MODIFY COLUMN id INT;
ALTER TABLE cnaes
    MODIFY COLUMN id INT;
ALTER TABLE empresas 
    MODIFY COLUMN natureza_juridica INT,
    MODIFY COLUMN qualificacao_do_responsavel INT;
ALTER TABLE estabelecimentos
    MODIFY COLUMN situacao_cadastral INT NOT NULL,
    MODIFY COLUMN motivo_situacao_cadastral INT,
    MODIFY COLUMN cnae_fiscal_principal INT,
    MODIFY COLUMN pais INT,
    MODIFY COLUMN municipio INT,
    MODIFY COLUMN ddd1 INT,
    MODIFY COLUMN telefone1 BIGINT,
    MODIFY COLUMN ddd2 INT,
    MODIFY COLUMN telefone2 BIGINT,
    MODIFY COLUMN ddd_fax INT,
    MODIFY COLUMN telefone_fax BIGINT;
ALTER TABLE estabelecimentos
    ADD CONSTRAINT FK_EstabSitCad FOREIGN KEY (situacao_cadastral) REFERENCES situacoes_cadastrais(id), 
    ADD CONSTRAINT FK_EstabMotivCad FOREIGN KEY (motivo_situacao_cadastral) REFERENCES motivos_de_situacoes_cadastrais(id),
    ADD CONSTRAINT FK_EstabPais FOREIGN KEY (pais) REFERENCES paises(id),
    ADD CONSTRAINT FK_EstabCnaePrinc FOREIGN KEY (cnae_fiscal_principal) REFERENCES cnaes(id),
    ADD CONSTRAINT FK_EstabMunic FOREIGN KEY (municipio) REFERENCES municipios(id);
ALTER TABLE empresas
    ADD CONSTRAINT FK_EmpNatJur FOREIGN KEY (natureza_juridica) REFERENCES naturezas_juridicas(id),
    ADD CONSTRAINT FK_EmpQualResp FOREIGN KEY (qualificacao_do_responsavel) REFERENCES qualificacoes_de_socios(id);