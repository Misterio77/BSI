use clap::{crate_authors, crate_version, App, AppSettings, Arg};

pub fn build_cli() -> App<'static> {
    App::new("oblig2")
        .about("Trabalho obrigatório 2")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("input")
                .about("Input RDF file (.rdf, .ttl, .nt)")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("output")
                .about("Output RDF file (.rdf, .ttl, .nt)")
                .value_name("FILE"),
        )
}
