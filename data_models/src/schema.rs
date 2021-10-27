table! {
    arquivos_importados (nome_do_arquivo) {
        nome_do_arquivo -> Varchar,
        tabela -> Varchar,
        registros_processados -> Unsigned<Integer>,
        tempo_decorrido_em_segundos -> Nullable<Unsigned<Bigint>>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    cnaes (id) {
        id -> Unsigned<Integer>,
        nome -> Varchar,
    }
}

table! {
    empresas (cnpj_basico) {
        cnpj_basico -> Char,
        razao_social -> Varchar,
        natureza_juridica -> Nullable<Unsigned<Smallint>>,
        qualificacao_do_responsavel -> Nullable<Unsigned<Tinyint>>,
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
        situacao_cadastral -> Unsigned<Tinyint>,
        data_situacao_cadastral -> Nullable<Date>,
        motivo_situacao_cadastral -> Nullable<Unsigned<Tinyint>>,
        nome_da_cidade_no_exterior -> Nullable<Varchar>,
        pais -> Nullable<Unsigned<Smallint>>,
        data_de_inicio_da_atividade -> Nullable<Date>,
        cnae_fiscal_principal -> Nullable<Unsigned<Integer>>,
        cnae_fiscal_secundaria -> Nullable<Varchar>,
        tipo_logradouro -> Nullable<Varchar>,
        logradouro -> Nullable<Varchar>,
        numero -> Nullable<Varchar>,
        complemento -> Nullable<Varchar>,
        bairro -> Nullable<Varchar>,
        cep -> Nullable<Varchar>,
        uf -> Nullable<Char>,
        municipio -> Nullable<Unsigned<Smallint>>,
        ddd1 -> Nullable<Varchar>,
        telefone1 -> Nullable<Varchar>,
        ddd2 -> Nullable<Varchar>,
        telefone2 -> Nullable<Varchar>,
        ddd_fax -> Nullable<Varchar>,
        telefone_fax -> Nullable<Varchar>,
        correio_eletronico -> Nullable<Varchar>,
        situacao_especial -> Nullable<Varchar>,
        data_situacao_especial -> Nullable<Date>,
    }
}

table! {
    faixas_etarias (id) {
        id -> Unsigned<Tinyint>,
        nome -> Nullable<Varchar>,
    }
}

table! {
    motivos_de_situacoes_cadastrais (id) {
        id -> Unsigned<Tinyint>,
        nome -> Varchar,
    }
}

table! {
    municipios (id) {
        id -> Unsigned<Smallint>,
        nome -> Varchar,
    }
}

table! {
    naturezas_juridicas (id) {
        id -> Unsigned<Smallint>,
        nome -> Varchar,
    }
}

table! {
    paises (id) {
        id -> Unsigned<Smallint>,
        nome -> Varchar,
    }
}

table! {
    qualificacoes_de_socios (id) {
        id -> Unsigned<Tinyint>,
        nome -> Varchar,
    }
}

table! {
    simples (cnpj_basico) {
        cnpj_basico -> Char,
        opcao_pelo_simples -> Char,
        data_de_opcao_pelo_simples -> Nullable<Date>,
        data_de_exclusao_do_simples -> Nullable<Date>,
        opcao_pelo_mei -> Char,
        data_de_opcao_pelo_mei -> Nullable<Date>,
        data_de_exclusao_do_mei -> Nullable<Date>,
    }
}

table! {
    situacoes_cadastrais (id) {
        id -> Unsigned<Tinyint>,
        nome -> Char,
    }
}

table! {
    socios (id) {
        id -> Unsigned<Integer>,
        cnpj_basico -> Char,
        identificador_de_socio -> Unsigned<Tinyint>,
        nome_ou_razao_social_do_socio -> Varchar,
        cnpj_ou_cpf_do_socio -> Nullable<Char>,
        qualificacao_do_socio -> Unsigned<Tinyint>,
        data_de_entrada_na_sociedade -> Date,
        pais_do_socio -> Nullable<Unsigned<Smallint>>,
        cpf_do_representante_legal -> Char,
        nome_do_representante_legal -> Varchar,
        qualificacao_do_representante_legal -> Unsigned<Tinyint>,
        faixa_etaria_do_socio -> Unsigned<Tinyint>,
    }
}

joinable!(empresas -> naturezas_juridicas (natureza_juridica));
joinable!(empresas -> qualificacoes_de_socios (qualificacao_do_responsavel));
joinable!(estabelecimentos -> cnaes (cnae_fiscal_principal));
joinable!(estabelecimentos -> empresas (cnpj_basico));
joinable!(estabelecimentos -> motivos_de_situacoes_cadastrais (motivo_situacao_cadastral));
joinable!(estabelecimentos -> municipios (municipio));
joinable!(estabelecimentos -> paises (pais));
joinable!(estabelecimentos -> situacoes_cadastrais (situacao_cadastral));

allow_tables_to_appear_in_same_query!(
    arquivos_importados,
    cnaes,
    empresas,
    estabelecimentos,
    faixas_etarias,
    motivos_de_situacoes_cadastrais,
    municipios,
    naturezas_juridicas,
    paises,
    qualificacoes_de_socios,
    simples,
    situacoes_cadastrais,
    socios,
);
