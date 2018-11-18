use git2::Repository;

type R<T> = Result<T, String>;

fn main() {
    println!("Hello, world!");
}

pub fn prompt(_repo: &Repository) -> R<Prompt> {
    Err("TODO".to_owned())
}

#[derive(Clone)]
pub struct Prompt {
    pub repo: RepoStatus,
    pub branch: BranchStatus,
    pub local: LocalStatus,
}

pub fn repo_status(_repo: &Repository) -> R<RepoStatus> {
    Err("TODO".to_owned())
}

#[derive(Clone)]
pub struct RepoStatus {
    pub branch: Option<String>,
    pub state: RepoState,
}

#[derive(Clone)]
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

#[derive(Clone, Default)]
pub struct BranchStatus {
    pub ahead: usize,
    pub behind: usize,
}

pub fn local_status(_repo: &Repository) -> R<LocalStatus> {
    Err("TODO".to_owned())
}

#[derive(Clone, Default)]
pub struct LocalStatus {
    pub staged: usize,
    pub unmerged: usize,
    pub unstaged: usize,
    pub untracked: usize,
}
