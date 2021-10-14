ALTER TABLE estabelecimentos MODIFY COLUMN situacao_cadastral TINYINT NOT NULL;
CREATE TABLE situacoes_cadastrais (
    id TINYINT PRIMARY KEY,
    nome CHAR(8) NOT NULL
);
INSERT INTO situacoes_cadastrais (id,nome) VALUES
    (1,"NULA"),
    (2,"ATIVA"),
    (3,"SUSPENSA"),
    (4,"INAPTA"),
    (8,"BAIXADA");
ALTER TABLE estabelecimentos
    ADD CONSTRAINT FK_EstabSitCad FOREIGN KEY (situacao_cadastral) REFERENCES situacoes_cadastrais(id);