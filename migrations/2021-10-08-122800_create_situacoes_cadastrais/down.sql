ALTER TABLE estabelecimentos DROP FOREIGN KEY FK_EstabSitCad;
DROP TABLE situacoes_cadastrais;
ALTER TABLE estabelecimentos MODIFY COLUMN situacao_cadastral CHAR(2) NOT NULL;