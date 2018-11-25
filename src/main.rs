extern crate clap;
use clap::{App, Arg};

mod model;
mod view;
use model::R;

fn main() {
    let matches = App::new("git_prompt")
        .version("v0.1")
        .author("aignas@github")
        .about("Prints your git prompt info fast!")
        .arg(
            Arg::with_name("test")
                .long("test")
                .help("print various combinations of the prompt"),
        )
        .arg(
            Arg::with_name("default_branch")
                .short("d")
                .long("default-branch")
                .help("default branch to use when printing diff status")
                .env("GIT_PROMPT_DEFAULT_BRANCH")
                .default_value("master"),
        )
        .arg(
            Arg::with_name("PATH")
                .help("Optional path to use for getting git info")
                .index(1)
                .default_value("."),
        )
        .get_matches();

    let c = view::DEFAULT_COLORS;
    let path = matches.value_of("PATH").unwrap();

    match git2::Repository::discover(path)
        .or_else(|e| Err(format!("{:?}", e)))
        .map(prompt)
    {
        Ok(p) => print!("{} ", view::PromptView::new(p, c)),
        Err(_) => print!(" "),
    }
}

pub fn prompt<T: model::Repo>(repo: T) -> R<model::Prompt> {
    let r = model::repo_status(&repo)?;
    let b = match &r.branch {
        Some(b) => model::branch_status(&repo, b, "master").ok(),
        None => None,
    };
    Ok(model::Prompt {
        repo: r,
        branch: b,
        local: model::local_status(&repo)?,
    })
}
