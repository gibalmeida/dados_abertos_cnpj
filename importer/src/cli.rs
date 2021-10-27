use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about="Importador de dados do CNPJ disponibilizado pela Receita Federal Brasil para um Banco de Dados MySQL.")]
pub struct Cli {
    // número de registros a ser inseridos ao mesmo tempo
    #[structopt(short,long,default_value="1000",help="Quantidade de registros a ser inseridos/alterados de uma única vez nas tabelas empresa, estabelecimentos, socios ou simples (nas outras tabelas, como são poucos registros, esta opção não tem efeito algum). Esta configuração pode aumentar ou diminuar a performance de inclusão/atualização de registros; porém, é necessário testar qual é o melhor valor a se utilizar. Valores muitos altos podem deixar lento o processamento, bem como gerar erros ao tentar incluir registros no banco de dados, pois vai depender da configuração do seu banco de dados.")]
    pub rows_per_insert: usize,
    #[structopt(short,long,help="Exibe algumas informações enquanto processa os registros.")]
    pub verbose: bool,
    #[structopt(short,long,help="Indica que a tabela está vazia e utiliza INSERT ao invés de REPLACE INTO. Deve ser utilizado apenas na primeira importação de cada tipo de tabela.")]
    pub empty: bool,
    #[structopt(short,long="drop-indexes",help="Antes de iniciar a importação do primeiro arquivo (Y0) das tabelas grandes (empresas, estabelecimentos, socios e simples) remove todos os índices, chaves primárias e chaves estrangeiras;e, ao processar o último arquivo (Y9) destas tabelas, é recriado tudo novamente. Este flag aumenta a velocidade de inserção de registros nas tabelas correspondentes e só deve ser utilizado quando estas tabelas estão sendo populadas novamente, ou seja, quando é feito INSERTs dos registros. No caso de utilizar este flag com as tabelas cheias, onde é utilizado REPLACE INTO para atualizar os registros, a perfomance de atualização caíra muito, pois não há índices para utilizar na busca dos registros.")]
    pub drop_indexes: bool,
    #[structopt(short,long,help="ATENÇÃO! Este flag zera a tabela (TRUNCATE TABLE) antes de importar os registros do primeiro arquivo (Y0) das tabelas grandes (empresas,estabelecimentos,socios e simples). Os arquivos devem ser importados em ordem das partes (Y0,Y1...), do contrário a tabela vai ser zerada quando chegar na parte (Y0).")]
    pub truncate_table: bool,
    #[structopt(parse(from_os_str))]
    pub file_to_import: std::path::PathBuf,

}
