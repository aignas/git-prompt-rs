use super::model::*;
use std::fmt::{self, Display, Formatter};

pub struct Colors {}

pub struct PromptView {
    pub repo: RepoStatusView,
    pub branch: BranchStatusView,
    pub local: LocalStatusView,
}

impl PromptView {
    pub fn new(p: Prompt, _c: &Colors) -> PromptView {
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
    pub model: Option<BranchStatus>,
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
