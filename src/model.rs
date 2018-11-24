type RepoState = git2::RepositoryState;

#[derive(Clone, Debug)]
pub struct Prompt {
    pub repo: RepoStatus,
    // it only makes sense to have the branch status when the repo is OK
    pub branch: Option<BranchStatus>,
    pub local: LocalStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepoStatus {
    // the branch might not be known, when the repo does not have any commits
    pub branch: Option<String>,
    pub state: RepoState,
}

#[derive(Clone, Debug, Default)]
pub struct BranchStatus {
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalStatus {
    pub staged: usize,
    pub unmerged: usize,
    pub unstaged: usize,
    pub untracked: usize,
}

impl LocalStatus {
    pub fn increment(&mut self, status: git2::Status) {
        if status.is_wt_new() {
            self.untracked += 1;
        }
        if status.is_index_new()
            || status.is_index_modified()
            || status.is_index_deleted()
            || status.is_index_renamed()
            || status.is_index_typechange()
        {
            self.staged += 1
        }
        if status.is_wt_modified()
            || status.is_wt_deleted()
            || status.is_wt_renamed()
            || status.is_wt_typechange()
        {
            self.unstaged += 1
        }
        if status.is_conflicted() {
            self.unmerged += 1
        }
    }
}

#[cfg(test)]
mod local_status {
    use super::*;

    fn get_status(s: git2::Status) -> LocalStatus {
        let mut actual = LocalStatus {
            ..Default::default()
        };
        actual.increment(s);
        actual
    }

    #[test]
    fn untracked() {
        let expected = LocalStatus {
            untracked: 1,
            ..Default::default()
        };
        assert_eq!(get_status(git2::Status::WT_NEW), expected);
    }
    #[test]
    fn staged() {
        let expected = LocalStatus {
            staged: 1,
            ..Default::default()
        };
        assert_eq!(get_status(git2::Status::INDEX_NEW), expected);
        assert_eq!(get_status(git2::Status::INDEX_MODIFIED), expected);
        assert_eq!(get_status(git2::Status::INDEX_DELETED), expected);
        assert_eq!(get_status(git2::Status::INDEX_RENAMED), expected);
        assert_eq!(get_status(git2::Status::INDEX_TYPECHANGE), expected);
    }
    #[test]
    fn unstaged() {
        let expected = LocalStatus {
            unstaged: 1,
            ..Default::default()
        };
        assert_eq!(get_status(git2::Status::WT_MODIFIED), expected);
        assert_eq!(get_status(git2::Status::WT_DELETED), expected);
        assert_eq!(get_status(git2::Status::WT_RENAMED), expected);
        assert_eq!(get_status(git2::Status::WT_TYPECHANGE), expected);
    }
    #[test]
    fn conflict() {
        let expected = LocalStatus {
            unmerged: 1,
            ..Default::default()
        };
        assert_eq!(get_status(git2::Status::CONFLICTED), expected);
    }
    #[test]
    fn partial_stage() {
        let expected = LocalStatus {
            staged: 1,
            unstaged: 1,
            ..Default::default()
        };
        let mut status = git2::Status::WT_MODIFIED;
        status.insert(git2::Status::INDEX_MODIFIED);
        assert_eq!(get_status(status), expected);
    }
}
