extern crate clap;
use clap::{App, Arg};

mod view;
use self::view::*;
mod model;
use self::model::*;

type R<T> = Result<T, String>;

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
    match get_prompt(path) {
        Ok(p) => print!("{} ", PromptView::new(p, Some(c))),
        Err(_) => print!(" "),
    }
}

pub trait Repo {
    fn state(&self) -> git2::RepositoryState;
    fn head(&self) -> Result<git2::Reference, git2::Error>;
    fn statuses(
        &self,
        options: Option<&mut git2::StatusOptions>,
    ) -> Result<git2::Statuses, git2::Error>;
    fn find_branch(
        &self,
        name: &str,
        branch_type: git2::BranchType,
    ) -> Result<git2::Branch, git2::Error>;
    fn revwalk(&self) -> Result<git2::Revwalk, git2::Error>;
}

impl Repo for git2::Repository {
    fn state(&self) -> git2::RepositoryState {
        self.state()
    }
    fn head(&self) -> Result<git2::Reference, git2::Error> {
        self.head()
    }
    fn statuses(
        &self,
        options: Option<&mut git2::StatusOptions>,
    ) -> Result<git2::Statuses, git2::Error> {
        self.statuses(options)
    }
    fn find_branch(
        &self,
        name: &str,
        branch_type: git2::BranchType,
    ) -> Result<git2::Branch, git2::Error> {
        self.find_branch(name, branch_type)
    }
    fn revwalk(&self) -> Result<git2::Revwalk, git2::Error> {
        self.revwalk()
    }
}

pub fn get_prompt(path: &str) -> R<Prompt> {
    let repo = git2::Repository::discover(path).or_else(|e| Err(format!("{:?}", e)))?;
    prompt(repo)
}

pub fn prompt<T: Repo>(repo: T) -> R<Prompt> {
    let r = repo_status(&repo)?;
    let b = match &r.branch {
        Some(b) => branch_status(&repo, b).ok(),
        None => None,
    };
    Ok(Prompt {
        repo: r,
        branch: b,
        local: local_status(&repo)?,
    })
}

pub fn repo_status(repo: &Repo) -> R<RepoStatus> {
    Ok(RepoStatus {
        branch: repo
            .head()
            .or_else(|e| Err(format!("{:?}", e)))
            .map(|r| r.shorthand().map(String::from))?,
        state: repo.state(),
    })
}

pub fn branch_status(repo: &Repo, name: &str) -> R<BranchStatus> {
    let name = get_remote_ref(repo, name).or_else(|_| get_remote_ref(repo, "master"))?;
    Ok(BranchStatus {
        ahead: diff(repo, &name, "HEAD")?,
        behind: diff(repo, "HEAD", &name)?,
    })
}

fn get_remote_ref(repo: &Repo, name: &str) -> R<String> {
    let br = repo
        .find_branch(name, git2::BranchType::Local)
        .or_else(|e| Err(format!("{:?}", e)))?;
    let upstream = br.upstream().or_else(|e| Err(format!("{:?}", e)))?;

    let reference = upstream.into_reference();
    reference
        .name()
        .ok_or("failed to get remote branch name".to_owned())
        .map(String::from)
}

fn diff(repo: &Repo, from: &str, to: &str) -> R<usize> {
    let mut revwalk = repo.revwalk().or_else(|e| Err(format!("{:?}", e)))?;
    revwalk
        .push_range(&format!("{}..{}", from, to))
        .or_else(|e| Err(format!("{:?}", e)))?;

    let c = revwalk.count();
    Ok(c)
}

pub fn local_status(repo: &Repo) -> R<LocalStatus> {
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(false)
        .renames_head_to_index(true);

    let statuses = repo
        .statuses(Some(&mut opts))
        .or_else(|e| Err(format!("{:?}", e)))?;

    let mut status = LocalStatus {
        ..Default::default()
    };
    for s in statuses.iter().map(|e| e.status()) {
        status.increment(s)
    }
    Ok(status)
}
