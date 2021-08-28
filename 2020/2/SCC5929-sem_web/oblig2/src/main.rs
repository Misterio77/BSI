use std::fs::{read_to_string, File};
use std::path::Path;

use anyhow::{anyhow, Context, Result};

use oxigraph::io::GraphFormat;
use oxigraph::model::{GraphName, NamedNode, Quad, Term};
use oxigraph::MemoryStore;

mod cli;

fn get_format(extension: &str) -> Option<GraphFormat> {
    match extension {
        "ttl" | "n3" => Some(GraphFormat::Turtle),
        "rdf" => Some(GraphFormat::RdfXml),
        "nt" => Some(GraphFormat::NTriples),
        _ => None,
    }
}

enum Age {
    Infant,
    Minor,
    Old,
}

fn has_age_class(quad: &Quad, class: Age) -> Option<bool> {
    if let Term::Literal(content) = &quad.object {
        if quad.predicate.as_str() == "http://xmlns.com/foaf/0.1/age" {
            let age: i32 = content.value().parse().ok()?;
            match class {
                Age::Infant => Some(age < 2),
                Age::Minor => Some(age < 18),
                Age::Old => Some(age > 70),
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();
    //Caminho para a entrada
    let in_path = Path::new(
        matches
            .value_of("input")
            .ok_or_else(|| anyhow!("Error getting input path"))?,
    );

    let out_path = Path::new(
        matches
            .value_of("output")
            .ok_or_else(|| anyhow!("Error getting output path"))?,
    );

    let in_extension = in_path
        .extension()
        .ok_or_else(|| anyhow!("The input specified is not a file with extension"))?
        .to_string_lossy()
        .into_owned();

    let out_extension = out_path
        .extension()
        .ok_or_else(|| anyhow!("The output specified is not a file extension"))?
        .to_string_lossy()
        .into_owned();

    let in_format =
        get_format(&in_extension).ok_or_else(|| anyhow!("Input file has an invalid format"))?;

    let out_format =
        get_format(&out_extension).ok_or_else(|| anyhow!("Output file has an invalid format"))?;

    let db = MemoryStore::new();

    db.load_graph(
        read_to_string(in_path)
            .context("Couldn't read input file")?
            .as_ref(),
        in_format,
        &GraphName::DefaultGraph,
        None,
    )?;

    let additional_info = "@prefix fam:   <http://www.ifi.uio.no/IN3060/family#> .
@prefix rdf:   <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix sim:   <http://www.ifi.uio.no/IN3060/simpsons#> .
@prefix xsd:   <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf:  <http://xmlns.com/foaf/0.1/> .

sim:Maggie	a				foaf:Person ;
			foaf:name		\"Maggie Simpson\" ;
			foaf:age		\"1\"^^xsd:int .

sim:Mona	a				foaf:Person ;
			foaf:name		\"Mona Simpson\" ;
			foaf:age		\"70\"^^xsd:int ;
			fam:hasSpouse	sim:Abraham .

sim:Abraham	a				foaf:Person ;
			foaf:name		\"Abraham Simpson\" ;
			foaf:age		\"78\"^^xsd:int ;
			fam:hasSpouse	sim:Mona .

sim:Herb	a				foaf:Person ;
			fam:hasFather	[] .";

    db.load_graph(
        additional_info.as_ref(),
        GraphFormat::Turtle,
        &GraphName::DefaultGraph,
        None,
    )?;

    let out_file = File::create(out_path).context("Couldn't create output file")?;
    db.dump_graph(out_file, out_format, &GraphName::DefaultGraph)?;

    for quad in db.iter() {
        let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
        if has_age_class(&quad, Age::Infant) == Some(true) {
            let object = NamedNode::new("http://www.ifi.uio.no/IN3060/family#Infant")?;
            db.insert(Quad::new(
                quad.subject.clone(),
                predicate.clone(),
                object,
                quad.graph_name.clone(),
            ))
        }
        if has_age_class(&quad, Age::Minor) == Some(true) {
            let object = NamedNode::new("http://www.ifi.uio.no/IN3060/family#Minor")?;
            db.insert(Quad::new(
                quad.subject.clone(),
                predicate.clone(),
                object,
                quad.graph_name.clone(),
            ))
        }
        if has_age_class(&quad, Age::Old) == Some(true) {
            let object = NamedNode::new("http://www.ifi.uio.no/IN3060/family#Old")?;
            db.insert(Quad::new(
                quad.subject.clone(),
                predicate.clone(),
                object,
                quad.graph_name.clone(),
            ))
        }
    }

    Ok(())
}
