use clap::{Args,Parser,Subcommand};

/// A CLI to get circleci jobs insights
#[derive(Debug, Parser)]
#[clap(name = "mig", version)]
pub struct App {
    /// Manage the configuration
    #[clap(subcommand)]
    command: Actions, 
}

#[derive(Debug,Subcommand)]
pub enum Actions {
  /// Retrieve migration information
  Analysis(Analysis),

  /// Manage the configuration
  Config(Config),
}

#[derive(Debug, Args)]
pub struct Analysis {
  /// Show migration data
  #[clap(short,long)]
  migration: ()
}

#[derive(Debug, Args)]
pub struct Config {
    /// Set the authorization field (circleci username)
    #[clap(short,long)]
    auth: Option<String>,

    /// Set the name of the followed project
    #[clap(short,long)]
    project: Option<String>,

    /// Set the slug of followed the project e.g. (github/space)
    #[clap(short,long)]
    slug: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::io::Cursor;

    #[allow(dead_code)]
    const EXPECTED_HELP: &str = r#"A CLI to get circleci jobs insights

Usage: mig <COMMAND>

Commands:
  analysis  Retrieve migration information
  config    Manage the configuration
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"#;

    #[test]
    fn test_help() {
        let mut app = App::command();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        app.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(help, EXPECTED_HELP);
    }

    const EXPECTED_CONFIG_HELP: &str = r#"Manage the configuration

Usage: config [OPTIONS]

Options:
  -a, --auth <AUTH>        Set the authorization field (circleci username)
  -p, --project <PROJECT>  Set the name of the followed project
  -s, --slug <SLUG>        Set the slug of followed the project e.g. (github/space)
  -h, --help               Print help
"#;

    #[test]
    fn test_config_help() {
        let mut app = App::command();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let read_cmd = app.find_subcommand_mut("config").unwrap();
        read_cmd.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(help, EXPECTED_CONFIG_HELP);
    }

    const EXPECTED_ANALYSIS_HELP: &str = r#"Retrieve migration information

Usage: analysis [OPTIONS]

Options:
  -m, --migration <MIGRATION>  Show migration data
  -h, --help                   Print help
"#;

    #[test]
    fn test_analysis_help() {
        let mut app = App::command();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let read_cmd = app.find_subcommand_mut("analysis").unwrap();
        read_cmd.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(help, EXPECTED_ANALYSIS_HELP);
    }
}