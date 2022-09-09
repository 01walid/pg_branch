use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use dotenv;
use eyre::eyre;
use pg_branch::PostgresManager;

use crate::config::Config;

use super::init::init_prompts;

pub struct Cli {
    parser: CliParser,
    pub config: Config,
    _pg_manager: Option<PostgresManager>,
}

/// Branch your development postgres database
#[derive(Parser, Debug)]
#[clap(name = "pg_branch")]
#[clap(version, about = "Branch your development postgres database", long_about = None)]
#[clap(propagate_version = true)]
pub struct CliParser {
    #[clap(subcommand)]
    command: Commands,
    // /// Name of the person to greet
    // #[clap(short, long)]
    // status: String,
}

#[derive(Subcommand, Debug, Eq, PartialEq)]
enum Commands {
    /// Initialize a new project
    #[clap()]
    Init {
        /// The remote to clone
        #[clap(short, long)]
        force: bool,
    },
    /// Switch to a fork database. Creates the database if it does not exist.
    #[clap()]
    Switch {
        /// Choose a specific target database
        #[clap(short, long, required = false, default_value = "<current_branch>")]
        target_db_name: String,

        /// Rename the new database to the same name as main database.
        #[clap(short, long)]
        rename: bool,
    },
    /// List forked databases
    #[clap(arg_required_else_help = true)]
    List {
        /// list forked databases
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    /// Drop a branch database
    #[clap(arg_required_else_help = true)]
    Drop {
        /// Database name
        #[clap(required = true, hide = true)]
        db_name: String,

        /// Do not prompt for a confirm message
        #[clap(short, long)]
        confirm: bool,
    },

    /// Ping the current main (template) database
    #[clap()]
    Ping {},
}

impl Cli {
    pub async fn new() -> Result<Cli> {
        let args = CliParser::parse();
        let config = Config::load()?;
        Ok(Cli {
            parser: args,
            config,
            _pg_manager: None,
        })
    }

    pub async fn ensure_db_conn(&mut self) -> Result<()> {
        self.load_from_dot_env();

        let db_url = env::var("DATABASE_URL")
            .map_err(|_| eyre!("DATABSE_URL environment variable not set"))?;

        let res = match &self._pg_manager {
            Some(_) => Ok(()),
            None => {
                let pg_manager = PostgresManager::new(db_url.as_str())
                    .await
                    .map_err(|_| eyre!("Failed to connect to the database!"))?;
                self._pg_manager = Some(pg_manager);
                Ok(())
            }
        };
        res
    }

    pub async fn ping(&mut self) -> Result<()> {
        let conn = self.pg_manager().await?;

        let res = conn.ping_db().await;
        if res.is_err() {
            return Err(eyre!("Failed to issue a ping to the database!"));
        }
        println!("Database successfully ping'ed!");
        Ok(())
    }

    pub async fn run() -> Result<()> {
        let mut cli = Self::new().await?;

        match cli.parser.command {
            Commands::Init { force } => {
                init_prompts(&mut cli, force).await?;
            }
            Commands::Switch {
                target_db_name,
                rename,
            } => {
                println!("Switching to {target_db_name}... ");
            }
            Commands::List { path } => {
                println!("Adding {:?}", path);
            }
            Commands::Drop { db_name, confirm } => {
                // pg_manager.drop_db(db_name);
            }
            Commands::Ping {} => cli.ping().await?,
        }
        Ok(())
    }

    pub async fn pg_manager(&mut self) -> Result<&PostgresManager> {
        self.ensure_db_conn().await?;
        let pg = match self._pg_manager.as_ref() {
            Some(value) => value,
            None => return Err(eyre!("Could not acquire a database connection!")),
        };

        Ok(pg)
    }

    pub fn load_from_dot_env(&self) {
        if self.config.load_dot_env {
            dotenv::dotenv().ok();
        }
    }
}
