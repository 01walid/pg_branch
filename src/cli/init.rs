use color_eyre::eyre::Result;
use console::style;
use inquire::{required, Confirm, Text};
use pg_branch::git;
// use std::io::prelude::*;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use super::Cli;

pub async fn init_prompts(cli: &mut Cli, force: bool) -> Result<()> {
    let add_to_gitignore = add_to_gitignore();
    let template_db = default_database()?;
    cli.config.template_db = template_db.clone();
    cli.config.git_main_branch = main_git_branch()?;
    cli.config.load_dot_env = auto_load_from_dotenv()?;

    cli.config.save()?;

    if add_to_gitignore {
        git_ignore_me().await?;
    }

    cli.pg_manager()
        .await?
        .mark_db_as_template(template_db.as_str())
        .await?;
    cli.pg_manager()
        .await?
        .fork_db("ondeck", "od-from-template-2", false)
        .await?;

    Ok(())
}

fn add_to_gitignore() -> bool {
    println!("This will add '.pg_branch' folder to the root of this repository");
    let ans = Confirm::new("Do you want to add '.pg_branch' to .gitignore?")
        .with_default(true)
        .with_help_message("It's for good reasons")
        .prompt();

    match ans {
        Ok(true) => {
            println!("{}", style("You're an awesome team player!\n").green());
            true
        }
        Ok(false) => {
            println!("Your choice! be careful '.pg_branch' folder is not meant to be shared.");
            false
        }
        Err(_) => panic!("Error with questionnaire, try again later"),
    }
}

fn default_database() -> Result<String> {
    let name = Text::new("Default database?")
        .with_default("dev")
        .with_help_message("This will be marked as the template database")
        .prompt()?;

    let msg = format!("From now on ░ {name} ░ is my idol!\n");
    println!("{}", style(msg).green());
    Ok(name)
}

fn auto_load_from_dotenv() -> Result<bool> {
    let ans = Confirm::new("Auto-load $DATABASE_URL from a '.env' file?")
        .with_help_message("If a .env file is not found this is a no-op")
        .prompt()?;

    Ok(ans)
}

fn main_git_branch() -> Result<String> {
    let ans = Text::new("Default git branch?")
        .with_help_message("Could be develop, main, or master. Depending your workflow.")
        .with_validators(&[required!()])
        .with_suggester(&suggest_branches)
        .prompt()?;

    println!(
        "{}{}{}",
        style("Phew! I can finally know where I'm ").green(),
        style("HEAD").white().bold(),
        style("ed ツ").green(),
    );

    Ok(ans)
}

fn suggest_branches(val: &str) -> Vec<String> {
    let repo = git::discover_repo(None).expect("This needs to be run inside a git repo!");
    let suggestions = git::repo_branches(&repo);

    let val_lower = val.to_lowercase();

    suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(s))
        .collect()
}

async fn git_ignore_me() -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(".gitignore")
        .await?;
    file.write_all(b"\n.pg_branch\n").await?;
    Ok(())
}
