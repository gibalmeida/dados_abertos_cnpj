CREATE TABLE arquivos_importados (
    nome_do_arquivo VARCHAR(255) NOT NULL PRIMARY KEY,
    tabela VARCHAR(100) NOT NULL,
    registros_processados INT UNSIGNED NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)