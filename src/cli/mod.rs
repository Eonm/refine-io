use clap::{App, Arg};

pub fn cli() -> clap::ArgMatches<'static> {
    App::new("refine-io")
        .version("1.0")
        .author("eonm. <eon.mathis@gmail.com>")
        .about("Automatise la creation et l'export de projets OpenRefine")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FICHIER|URL")
                .help("Source des données")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Format des données d'import")
                .possible_values(&["json", "xml", "csv", "tsv", "xls", "xlsx"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("record_path")
                .short("r")
                .long("record-path")
                .value_name("FILE")
                .help("Expression pour l'analyse des données")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("project_name")
                .short("n")
                .long("project-name")
                .value_name("NAME")
                .conflicts_with("input")
                .help("Nom du projet OpenRefine")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("open_project")
                .long("open-project")
                .help("Ouvre le projet OpenRefine dans le navigateur"),
        )
        .arg(
            Arg::with_name("project_id")
                .long("project-id")
                .value_name("ID")
                .conflicts_with("input")
                .takes_value(true)
                .help("Charge un projet OpenRefine"),
        )
        .arg(
            Arg::with_name("script")
                .short("s")
                .long("script")
                .value_name("FILE")
                .help("Applique un script de transformation OpenRefine")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sync")
                .long("sync")
                .help("Attend que les scripts aient fini d'être appliqués avant d'exporter ou d'afficher les données du projet OpenRefine")
                .requires("script"),
        )
        .arg(
            Arg::with_name("export")
                .short("e")
                .long("export")
                .value_name("FORMAT")
                .help("Format d'export")
                .possible_values(&["csv", "tsv", "xls", "xlsx", "ods", "html"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("open_export")
                .long("open-export")
                .requires("export")
                .help("Ouvre les données téléchargées"),
        )
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .value_name("FORMAT")
                .takes_value(true)
                .possible_values(&["csv", "tsv", "html"])
                .help("Affiche les données du projet OpenRefine"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output-name")
                .value_name("FICHIER")
                .takes_value(true)
                .help("Nom du fichier exporté par OpenRefine"),
        )
        .arg(
            Arg::with_name("clean")
                .visible_alias("delete")
                .short("C")
                .long("clean")
                .conflicts_with("open_project")
                .help("Supprime le projet OpenRefine"),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .help("Cache les logs"),
        )
        .get_matches()
}
