use git2::Repository;

mod view;
use self::view::*;
mod model;
use self::model::*;

type R<T> = Result<T, String>;

fn main() {
    let c = Colors {};
    match get_prompt("") {
        Ok(p) => print!("{} ", PromptView::new(p, &c)),
        Err(_) => print!(" "),
    }
}

pub fn get_prompt(path: &str) -> R<Prompt> {
    let repo = Repository::discover(path).or_else(|e| Err(format!("{:?}", e)))?;
    prompt(repo)
}

pub fn prompt(repo: Repository) -> R<Prompt> {
    Ok(Prompt {
        repo: repo_status(&repo)?,
        branch: branch_status(&repo).ok(),
        local: local_status(&repo)?,
    })
}

pub fn repo_status(_repo: &Repository) -> R<RepoStatus> {
    Err("TODO".to_owned())
}

pub fn branch_status(_repo: &Repository) -> R<BranchStatus> {
    Err("TODO".to_owned())
}

pub fn local_status(_repo: &Repository) -> R<LocalStatus> {
    Err("TODO".to_owned())
}
