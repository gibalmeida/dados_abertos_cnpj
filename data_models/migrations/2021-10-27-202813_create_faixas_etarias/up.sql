CREATE TABLE faixas_etarias (
    id TINYINT UNSIGNED PRIMARY KEY,
    nome VARCHAR(30)
);
INSERT INTO faixas_etarias (id,nome) VALUES
   (0, "Não se aplica"),
   (1, "Entre 0 a 12 anos"),
   (2, "Entre 13 a 20 anos"),
   (3, "Entre 21 a 30 anos"),
   (4, "Entre 31 a 40 anos"),
   (5, "Entre 41 a 50 anos"),
   (6, "Entre 51 a 60 anos"),
   (7, "Entre 61 a 70 anos"),
   (8, "Entre 71 a 80 anos"),
   (9, "Maiores de 80 anos");