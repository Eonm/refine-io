use clap::{App, Arg};

pub fn cli() -> clap::ArgMatches<'static> {
    App::new("refine-io")
        .version("1.0")
        .author("eonm. <eon.mathis@gmail.com>")
        .about("Automatise la creation et l'export de projet OpenRefine")
        .arg(
            Arg::with_name("script")
                .short("s")
                .long("script")
                .value_name("FILE")
                .help("Script de transformation OpenRefine")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("record_path")
                .short("r")
                .long("record-path")
                .value_name("FILE")
                .help("")
                // .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project_name")
                .short("n")
                .long("project-name")
                .value_name("NAME")
                .help("Nom du projet OpenRefine")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Fichier de sortie")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Format de fichier")
                .possible_values(&["json", "xml", "csv", "tsv", "xls", "xlsx"])
                // .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export_format")
                .short("e")
                .long("export-format")
                .value_name("FORMAT")
                .help("Format d'export")
                .possible_values(&["csv", "tsv", "xls", "xlsx", "ods", "html"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FICHIER|URL")
                .help("Source")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("open_project")
                .long("open-project")
                .help("Ouvre le projet OpenRefine après sa création"),
        )
        .arg(
            Arg::with_name("open_result")
                .long("open-result")
                //require export
                .help("Ouvre les données téléchargées"),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .help("N'affiche pas les logs"),
        )
        .arg(
            Arg::with_name("project_id")
                .long("project-id")
                .conflicts_with("input")
                .takes_value(true)
                .help("Édite ou supprime un projet par son ID"),
        )
        .get_matches()
}
