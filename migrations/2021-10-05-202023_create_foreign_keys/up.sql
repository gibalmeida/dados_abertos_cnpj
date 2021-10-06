ALTER TABLE estabelecimentos 
    ADD CONSTRAINT FK_EstabEmp FOREIGN KEY (cnpj_basico) REFERENCES empresas(cnpj_basico),
    ADD CONSTRAINT FK_EstabMotivCad FOREIGN KEY (motivo_situacao_cadastral) REFERENCES motivos_de_situacoes_cadastrais(id),
    ADD CONSTRAINT FK_EstabPais FOREIGN KEY (pais) REFERENCES paises(id),
    ADD CONSTRAINT FK_EstabCnaePrinc FOREIGN KEY (cnae_fiscal_principal) REFERENCES cnaes(id),
    ADD CONSTRAINT FK_EstabMunic FOREIGN KEY (municipio) REFERENCES municipios(id);
ALTER TABLE empresas
    ADD CONSTRAINT FK_EmpNatJur FOREIGN KEY (natureza_juridica) REFERENCES naturezas_juridicas(id),
    ADD CONSTRAINT FK_EmpQualResp FOREIGN KEY (qualificacao_do_responsavel) REFERENCES qualificacoes_de_socios(id);