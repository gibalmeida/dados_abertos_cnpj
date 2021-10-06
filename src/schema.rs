table! {
    cnaes (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    empresas (cnpj_basico) {
        cnpj_basico -> Char,
        razao_social -> Varchar,
        natureza_juridica -> Nullable<Integer>,
        qualificacao_do_responsavel -> Nullable<Integer>,
        capital_social -> Nullable<Decimal>,
        porte -> Nullable<Char>,
        ente_federativo_responsavel -> Nullable<Varchar>,
    }
}

table! {
    estabelecimentos (cnpj_basico, cnpj_ordem, cnpj_dv) {
        cnpj_basico -> Char,
        cnpj_ordem -> Char,
        cnpj_dv -> Char,
        identificador_matriz_filial -> Char,
        nome_fantasia -> Nullable<Varchar>,
        situacao_cadastral -> Char,
        data_situacao_cadastral -> Nullable<Date>,
        motivo_situacao_cadastral -> Nullable<Integer>,
        nome_da_cidade_no_exterior -> Nullable<Varchar>,
        pais -> Nullable<Integer>,
        data_de_inicio_da_atividade -> Nullable<Date>,
        cnae_fiscal_principal -> Nullable<Integer>,
        cnae_fiscal_secundaria -> Nullable<Varchar>,
        tipo_logradouro -> Nullable<Varchar>,
        logradouro -> Nullable<Varchar>,
        numero -> Nullable<Varchar>,
        complemento -> Nullable<Varchar>,
        bairro -> Nullable<Varchar>,
        cep -> Nullable<Varchar>,
        uf -> Nullable<Char>,
        municipio -> Nullable<Integer>,
        ddd1 -> Nullable<Integer>,
        telefone1 -> Nullable<Bigint>,
        ddd2 -> Nullable<Integer>,
        telefone2 -> Nullable<Bigint>,
        ddd_fax -> Nullable<Integer>,
        telefone_fax -> Nullable<Bigint>,
        correio_eletronico -> Nullable<Varchar>,
        situacao_especial -> Nullable<Varchar>,
        data_situacao_especial -> Nullable<Date>,
    }
}

table! {
    motivos_de_situacoes_cadastrais (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    municipios (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    naturezas_juridicas (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    paises (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

table! {
    qualificacoes_de_socios (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

joinable!(empresas -> naturezas_juridicas (natureza_juridica));
joinable!(empresas -> qualificacoes_de_socios (qualificacao_do_responsavel));
joinable!(estabelecimentos -> cnaes (cnae_fiscal_principal));
joinable!(estabelecimentos -> empresas (cnpj_basico));
joinable!(estabelecimentos -> motivos_de_situacoes_cadastrais (motivo_situacao_cadastral));
joinable!(estabelecimentos -> municipios (municipio));
joinable!(estabelecimentos -> paises (pais));

allow_tables_to_appear_in_same_query!(
    cnaes,
    empresas,
    estabelecimentos,
    motivos_de_situacoes_cadastrais,
    municipios,
    naturezas_juridicas,
    paises,
    qualificacoes_de_socios,
);
