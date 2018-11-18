use git2::Repository;
use std::fmt::{self, Display, Formatter};

type R<T> = Result<T, String>;

fn main() {
    let c = Colors {};
    match display_prompt("", &c) {
        Ok(p) => print!("{} ", p),
        Err(_) => print!(" "),
    }
}

pub struct Colors {}

pub fn display_prompt(path: &str, c: &Colors) -> R<PromptView> {
    let repo = Repository::discover(path).or_else(|e| Err(format!("{:?}", e)))?;
    prompt(repo).map(|p| PromptView::new(p, c))
}

pub struct PromptView {
    pub repo: RepoStatusView,
    pub branch: BranchStatusView,
    pub local: LocalStatusView,
}

impl PromptView {
    fn new(p: Prompt, _c: &Colors) -> PromptView {
        PromptView {
            repo: RepoStatusView { model: p.repo },
            branch: BranchStatusView { model: p.branch },
            local: LocalStatusView { model: p.local },
        }
    }
}

impl Display for PromptView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.repo, self.branch, self.local)
    }
}

pub struct RepoStatusView {
    pub model: RepoStatus,
}

impl Display for RepoStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.model)
    }
}
pub struct BranchStatusView {
    pub model: BranchStatus,
}

impl Display for BranchStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.model)
    }
}
pub struct LocalStatusView {
    pub model: LocalStatus,
}

impl Display for LocalStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.model)
    }
}

pub fn prompt(repo: Repository) -> R<Prompt> {
    Ok(Prompt {
        repo: repo_status(&repo)?,
        branch: branch_status(&repo)?,
        local: local_status(&repo)?,
    })
}

#[derive(Clone, Debug)]
pub struct Prompt {
    pub repo: RepoStatus,
    pub branch: BranchStatus,
    pub local: LocalStatus,
}

pub fn repo_status(_repo: &Repository) -> R<RepoStatus> {
    Err("TODO".to_owned())
}

#[derive(Clone, Debug)]
pub struct RepoStatus {
    pub branch: Option<String>,
    pub state: RepoState,
}

#[derive(Clone, Debug)]
pub enum RepoState {
    CherryPick,
    Merge,
    New,
    OK,
    Rebase,
    Revert,
}

pub fn branch_status(_repo: &Repository) -> R<BranchStatus> {
    Err("TODO".to_owned())
}

#[derive(Clone, Debug, Default)]
pub struct BranchStatus {
    pub ahead: usize,
    pub behind: usize,
}

pub fn local_status(_repo: &Repository) -> R<LocalStatus> {
    Err("TODO".to_owned())
}

#[derive(Clone, Debug, Default)]
pub struct LocalStatus {
    pub staged: usize,
    pub unmerged: usize,
    pub unstaged: usize,
    pub untracked: usize,
}
