use clap::{crate_authors, crate_version, App, AppSettings, Arg};

pub fn build_cli() -> App<'static> {
    App::new("oblig3")
        .about("Trabalho obrigatório 3")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("graph")
                .about("RDF Graph file (.ttl)")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("query")
                .about("SPARQL Queries file (.rq)")
                .value_name("FILE"),
        )
}
