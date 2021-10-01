# Dados Públicos CNPJ

Este projeto foi criado com o objetivo de importar para um banco de dados (MySQL) os arquivos (CSV) com os dados públicos dos cadastros de CNPJ que são disponibilizados pela Receita Federal do Brasil, e, futuramente, exportar estes dados por meio de uma API (REST).

Abaixo você encontrará as instruções com os passos preparativos para a importação dos dados e como fazer para importá-los para o banco de dados. Os comandos mostrados abaixo assumem que você estará utilizando uma distribuição do Sistema Operacional Linux (no momento do desenvolvimento deste projeto eu estava utilizando a Distro Ubuntu 20.04); então se você estiver utilizando outro S.O.(Windows ou MacOS X) talvez você precisará adaptar alguns comandos (ai é com você).

>Nota: Este é o meu primeiro projeto em Rust, então o código não está bem estruturado... ainda.

## Preparativos para a importação dos arquivos

### 1. Baixe os arquivos da CSV da RF

Você vai precisar dos arquivos com os dados públicos do CNPJ disponibilizados pela Receita Federal do Brasil, os quais você poderá baixa da página deste [link](https://www.gov.br/receitafederal/pt-br/assuntos/orientacao-tributaria/cadastros/consultas/dados-publicos-cnpj).

>Seja muito paciente para baixar os arquivos, pois o site é bem lento e como são vários arquivos, alguns bem grandes, vai demorar bastante. Eu recomendo que, ao invés de utilizar o navegador, se utilize alguma ferramenta para baixar estes arquivos. No meu caso eu utilizo o comando `wget` com parâmetro `--tries=0` (tentativas infinitas) para cada URL dos arquivos disponibilizados; daí e deixo o comando rodando até que ele consiga fazer o downloads de todos os arquivos. Quem sabe?!? Talvez, no futuro eu faça um script que rode no cron, e que baixe todos os arquivos uma vez por mês (todo mes tem atualizaçã) e importe todos eles novamente para o banco de dados.

### 2. Instale o Rust

Se não tiver o Rust instaldo no seu S.O, você precisará instalar ele para conseguir fazer o importador funcionar.

Para instalar o Rust é só seguir as instruções na página [deste link](https://www.rust-lang.org/pt-BR/tools/install).

### 3. Instale o Diesel CLI

Depois de instalado o Rust, e se ele estiver funcionando corretamente, você precisa instalar o Diesel CLI. É só executar o seguinte comando:

```bash
cargo install diesel_cli
```

>Você poderá encontrar mais informações sobre o Diesel no seguinte [link](https://diesel.rs/guides/getting-started).

### 4. Crie o banco de dados

Aqui o primeiro passo é editar o arquivo `.env` e configurar os dados da conexão com o banco de dados na variavel de ambiente `DATABASE_URL`, conforme exemplo a seguir:

```bash
DATABASE_URL=mysql://usuario:senha@endereco.do.servidor.mysql:porta/nome_do_banco_de_dados_mysql
```

Depois, se você ainda não criou o banco de dados, crie ele e dê ao usuário que você especificou na variável de ambiente `DATABASE_URL` permissões para acessar e criar tabelas (para não ter dores de cabeça, dê todas as permissões no banco de dados para o usuário que você especificar, ou utilize o usuário `root`).

E por fim, para criar as tabelas necessárias no banco de dados, execute o seguinte comando:

```bash
diesel migration run
```

## Importação dos dados

A seguir são apresentado duas formas de executar o comando para a importação dos dados: com os arquivos CSVs descompactados em uma pasta; ou sem descompactá-los na pasta.

>Antes de executar os comandos abaixo no seu terminal, você deve ir (`cd`) para  está dentro da pasta `dados_abertos_cnpj` (pasta do projeto). Ex: `cd /meusprojetos/dados_abertos_cnpj`

### 1. Arquivos descompactados

Nesta forma de importação você primeiro descompacta todos os arquivos que baixou da RF utilizando o seu descompactador de arquivos ZIP preferido. (No Linux eu utilizo o `unzip`.)

Para cada arquivo descompactado, para importar os dados para o banco de dados, você irá executar o comando da seguinte forma:

```bash
cargo run TABELA < CAMINHO_DO_ARQUIVO_CSV
```

Onde:

- __TABELA__ deverá ser substituído por uma das seguintes tabelas abaixo, conforme a terminação do nome do arquivo (indicado entre parentes) que será processado pelo comando:

  - empresas (.EMPRECSV)
  - estabelecimentos (.ESTABELE)
  - paises (.PAISCSV)
  - municipios (.MUNICCSV)
  - naturezas_juridicas (.NATJUCSV)
  - cnaes (.CNAECSV)
  - qualificacoes_de_socios (QUALSCSV)
  - motivos_situacao_cadastral (.MOTICSV) (ainda não implementado)
  - simples (.SIMPLES.CSV) (ainda não implementado)
  - socioes (.???) (ainda não implementado)

- __CAMINHO_DO_ARQUIVO_CSV__ deverá ser substituído pelo caminho completo do arquivo descompactado que será processado pelo comando (ex: `/home/user/Downloads/K3241.K03200Y0.D10911.ESTABELE`).

### 2. Arquivos compactados

Nesta forma de importação, a saída (_stdout_) do comando que descompacta  o arquivo (`unzip`) envia os dados descompactados, via _pipe_ (|), para a entrada (_stdin_) do comando que faz a importação. Portanto, não há necessidade de espaço em disco para salvar o arquivo descompactado, já que é feito tudo em um único passo.
> Caso o seu Linux não possua o comando `unzip` intalado, você precisará instalar ele antes de utilizar no comando abaixo.

A sintaxe do comando de importação é praticamente a mesma da apresentada na forma de importação de arquivos descompactados. Segue o formato do comando:

```bash
unzip -p CAMINHO_DO_ARQUIVO_CSV_COMPACTADO | cargo run TABELA
```

Onde:

- __TABELA__  ídem a apresentada na forma dos arquivos descompactados

- __CAMINHO_DO_ARQUIVO_CSV_COMPACTADO__  deverá ser substituído pelo caminho completo do arquivo compactado que será processado pelo comando (ex: `/home/user/Downloads/K3241.K03200Y0.D10911.ESTABELE.zip`).
