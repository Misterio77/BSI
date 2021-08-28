use std::fs::read_to_string;

use oxigraph::io::{GraphFormat, GraphSerializer};
use oxigraph::model::GraphName;
use oxigraph::sparql;
use oxigraph::MemoryStore;

use prettytable::{Cell, Row, Table};

use anyhow::{anyhow, Context, Result};

mod cli;

fn solutions_values(solutions: sparql::QuerySolutionIter) -> Result<Vec<Vec<String>>> {
    //Vetor para retornarmos
    let mut vec: Vec<Vec<String>> = Vec::new();
    //Variáveis pedidas na query, usaremos como a primeira linha da tabela
    let mut variables: Vec<String> = Vec::new();

    //Iterar pelas variáveis
    for variable in solutions.variables() {
        variables.push(variable.to_string().replace('?', ""));
    }

    //Clonando para ainda poder acessar depois
    vec.push(variables.clone());

    //Iterar pelos resultados
    for solution in solutions {
        let solution = solution?;
        let mut row: Vec<String> = Vec::new();
        //Iterar pelos campos de cada resultado
        for variable in &variables {
            //Caso tenha um campo para colocar
            if let Some(content) = solution.get(variable.as_str()) {
                row.push(format!("{}", content));
            } else {
                row.push(String::from(""));
            }
        }
        //Colocar a linha (cada uma equivale a um resultado) no vetor
        vec.push(row);
    }
    //Devolver vetor
    Ok(vec)
}

fn convert_triples(triples: sparql::QueryTripleIter, format: GraphFormat) -> Result<String> {
    //Criar uma string, transformar em bytes e tomar posse
    let mut buffer = String::new().as_bytes().to_owned();
    //Inicializar um escritor do serializador de grafos, no formato dado para o buffer
    let mut writer = GraphSerializer::from_format(format).triple_writer(&mut buffer)?;
    //Iterar
    for item in triples {
        //Escrever a tripla
        writer.write(item?.as_ref())?;
    }
    //Finalizar
    writer.finish()?;
    //Transformar novamente em string e devolver
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

fn print_query(query: sparql::QueryResults) -> Result<()> {
    match query {
        //Caso seja do tipo ASK
        sparql::QueryResults::Boolean(boolean) => {
            //Imprimir o boolean
            println!("{}", boolean)
        }
        //Caso seja do tipo SELECT
        sparql::QueryResults::Solutions(solutions) => {
            //Obter os valores da query em um vetor de vetor de string
            let values = solutions_values(solutions)?;
            //Montar uma tabela
            let mut table = Table::new();
            //Para cada linha
            for values_row in values {
                //Criar linha vazia
                let mut row = Row::empty();
                for value in values_row {
                    //Colocar cada celula
                    row.add_cell(Cell::new(&value));
                }
                //E colocar a linha completa na tabela
                table.add_row(row);
            }

            //Imprimir a tabela no stdout
            table.printstd();
        }
        //Caso seja do tipo CONSTRUCT ou DESCRIBE
        sparql::QueryResults::Graph(graph) => {
            //Transformar as triplas do grafo em turtle
            let output = convert_triples(graph, GraphFormat::Turtle)?;
            //Imprimir
            println!("{}", output);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    //Obter os arguemntos de CLI
    let matches = cli::build_cli().get_matches();
    //Caminho para a .ttl
    let graph_path = matches
        .value_of("graph")
        .ok_or_else(|| anyhow!("Error getting graph path"))?;
    //Caminho para a .rq
    let query_path = matches
        .value_of("query")
        .ok_or_else(|| anyhow!("Error getting query path"))?;

    //Ler ambos os arquivos pra uma string
    let graph_str = read_to_string(graph_path).with_context(|| "Couldn't read graph file")?;
    let query_str = read_to_string(query_path).with_context(|| "Couldn't read query file")?;

    let query = sparql::Query::parse(&query_str, None)?;

    //Criar uma SPARQL na memória mesmo
    let db = MemoryStore::new();

    //Carregar o grafo
    db.load_graph(
        graph_str.as_ref(),
        GraphFormat::Turtle,
        &GraphName::DefaultGraph,
        None,
    )?;

    //Fazer a query especificada pelo usuário
    let query_result = db
        .query(query, sparql::QueryOptions::default())
        .with_context(|| "Invalid query")?;

    print_query(query_result)?;

    Ok(())
}
