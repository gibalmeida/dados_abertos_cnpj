use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about="Importador de dados do CNPJ disponibilizado pela Receita Federal Brasil para um Banco de Dados MySQL.")]
pub struct Cli {
    // número de registros a ser inseridos ao mesmo tempo
    #[structopt(short="r",long="rows-per-insert",default_value="1000",help="Quantidade de registros a ser inseridos/alterados de uma única vez nas tabelas empresa, estabelecimentos, socios ou simples (nas outras tabelas, como são poucos registros, esta opção não tem efeito algum). Esta configuração pode aumentar ou diminuar a performance de inclusão/atualização de registros; porém, é necessário testar qual é o melhor valor a se utilizar. Valores muitos altos podem deixar lento o processamento, bem como gerar erros ao tentar incluir registros no banco de dados, pois vai depender da configuração do seu banco de dados.")]
    pub rows_per_insert: usize,
    #[structopt(short="v",long="verbose",help="Exibe algumas informações enquanto processa os registros.")]
    pub verbose: bool,
    #[structopt(short="e",long="empty",help="Indica que a tabela está vazia e utiliza INSERT ao invés de REPLACE INTO. Deve ser utilizado apenas na primeira importação de cada tipo de tabela.")]
    pub empty: bool,
    #[structopt(short="d",long="disable-keys",help="Antes de iniciar a importação desativa as chaves da tabela. Ao final da importação habilita as chaves novamente")]
    pub disable_keys: bool,
    #[structopt(short="t",long="truncate-table",help="ATENÇÃO! Este flag zera a tabela (TRUNCATE TABLE) antes de importar os registros do primeiro arquivo (Y0) das tabelas grandes (empresas,estabelecimentos,socios e simples). Os arquivos devem ser importados em ordem das partes (Y0,Y1...), do contrário a tabela vai ser zerada quando chegar na parte (Y0).")]
    pub truncate_table: bool,
    #[structopt(parse(from_os_str))]
    pub file_to_import: std::path::PathBuf,

}
