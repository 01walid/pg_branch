use std::{env, path::PathBuf};

use color_eyre::eyre::Result;
use eyre::{eyre, ContextCompat};
use git2::{Branch, BranchType, Repository};

pub fn discover_repo(path: Option<&PathBuf>) -> Result<Repository> {
    // TODO: refactor this.
    let cd = &env::current_dir()?;
    let p = path.unwrap_or(cd);
    let repo = Repository::discover(p).expect("pg_branch needs to be run inside a git repo");
    Ok(repo)
}

pub fn repo_root_path(path: Option<&PathBuf>) -> Result<PathBuf> {
    let repo = discover_repo(path)?;
    let repo_root = repo
        .workdir()
        .expect("bare git repositories are not supported");

    Ok(repo_root.into())
}

pub fn current_branch(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    if head.is_branch() {
        let branch_name = head
            .shorthand()
            .wrap_err("Cloud not get the current branch name")?;

        return Ok(branch_name.into());
    }
    Err(eyre!("Current git head is not pointing at a branch. Are you checking out a tag or a specific commit?"))
}

pub fn repo_branches(repo: &Repository) -> Vec<String> {
    let mut res: Vec<String> = vec![];

    if let Ok(branches) = repo.branches(Some(BranchType::Local)) {
        for b in branches {
            if let Ok((branch, _)) = b {
                let branch_name = _extract_branch_name_or_empty(&branch);
                if !branch_name.is_empty() {
                    res.push(branch_name)
                }
            }
        }
    }
    res
}

fn _extract_branch_name_or_empty(branch: &Branch) -> String {
    let bn = branch.name();
    if let Ok(branch_name) = bn {
        if let Some(bn) = branch_name {
            return bn.into();
        }
    }
    "".into()
}
