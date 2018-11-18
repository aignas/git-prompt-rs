#[derive(Clone, Debug)]
pub struct Prompt {
    pub repo: RepoStatus,
    // it only makes sense to have the branch status when the repo is OK
    pub branch: Option<BranchStatus>,
    pub local: LocalStatus,
}

#[derive(Clone, Debug)]
pub struct RepoStatus {
    // the branch might not be known, when the repo does not have any commits
    pub branch: Option<String>,
    pub state: RepoState,
}

#[derive(Clone, Debug)]
pub enum RepoState {
    New,
    OK,

    CherryPick,
    Merge,
    Rebase,
    Revert,
}

#[derive(Clone, Debug, Default)]
pub struct BranchStatus {
    pub ahead: usize,
    pub behind: usize,
}
#[derive(Clone, Debug, Default)]
pub struct LocalStatus {
    pub staged: usize,
    pub unmerged: usize,
    pub unstaged: usize,
    pub untracked: usize,
}
