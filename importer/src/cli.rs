use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about="Importador de dados do CNPJ disponibilizado pela Receita Federal Brasil para um Banco de Dados MySQL.")]
pub struct Cli {
    // número de registros a ser inseridos ao mesmo tempo
    #[structopt(short="r",long="rows-per-insert",default_value="1000",help="Quantidade de registros a ser inseridos/alterados de uma única vez nas tabelas empresa, estabelecimentos, socios ou simples (nas outras tabelas, como são poucos registros, esta opção não tem efeito algum). Esta configuração pode aumentar ou diminuar a performance de inclusão/atualização de registros; porém, é necessário testar qual é o melhor valor a se utilizar. Valores muitos altos podem deixar lento o processamento, bem como gerar erros ao tentar incluir registros no banco de dados, pois vai depender da configuração do seu banco de dados.")]
    pub rows_per_insert: usize,
    #[structopt(short="v",long="verbose",help="Exibe algumas informações enquanto processa os registros.")]
    pub verbose: bool,
    #[structopt(parse(from_os_str))]
    pub file_to_import: std::path::PathBuf,

}
