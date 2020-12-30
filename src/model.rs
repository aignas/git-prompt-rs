type RepoState = git2::RepositoryState;
pub type R<T> = Result<T, String>;

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
    pub fn new() -> LocalStatus {
        LocalStatus {
            ..Default::default()
        }
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
    fn graph_ahead_behind(
        &self,
        local: git2::Oid,
        upstream: git2::Oid,
    ) -> Result<(usize, usize), git2::Error>;
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
    fn graph_ahead_behind(
        &self,
        local: git2::Oid,
        upstream: git2::Oid,
    ) -> Result<(usize, usize), git2::Error> {
        self.graph_ahead_behind(local, upstream)
    }
}

pub fn repo_status(repo: &dyn Repo) -> R<RepoStatus> {
    Ok(RepoStatus {
        branch: repo
            .head()
            .or_else(|e| Err(format!("{:?}", e)))
            .map(|r| get_repo_rev(&r))?,
        state: repo.state(),
    })
}

fn get_repo_rev(r: &dyn Reference) -> Option<String> {
    match r.shorthand() {
        Some("HEAD") => r.short_id().ok(), // TODO don't discard error
        Some(b) => Some(b.into()),
        None => None,
    }
}

pub trait Reference {
    fn shorthand(&self) -> Option<&str>;
    fn short_id(&self) -> Result<String, String>;
    fn target(&self) -> Option<git2::Oid>;
}

impl<'repo> Reference for git2::Reference<'repo> {
    fn shorthand(&self) -> Option<&str> {
        self.shorthand()
    }
    fn short_id(&self) -> Result<String, String> {
        self.peel_to_commit()
            .or_else(|e| Err(format!("{:?}", e)))?
            .as_object()
            .short_id()
            .or_else(|e| Err(format!("{:?}", e)))?
            .as_str()
            .ok_or_else(|| "invalid utf-8".to_string())
            .and_then(|s| Ok(s.into()))
    }
    fn target(&self) -> Option<git2::Oid> {
        self.target()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod repo_status {
    use super::*;

    struct TestReference<'a> {
        shorthand: Option<&'a str>,
        short_id: Option<&'a str>,
        target: Option<git2::Oid>,
    }

    impl<'a> Reference for TestReference<'a> {
        fn shorthand(&self) -> Option<&str> {
            self.shorthand
        }
        fn target(&self) -> Option<git2::Oid> {
            self.target
        }
        fn short_id(&self) -> Result<String, String> {
            Ok(self.short_id.unwrap().to_string())
        }
    }

    #[test]
    fn get_shorthand() {
        let r = TestReference {
            shorthand: Some("foo"),
            short_id: Some("ha"),
            target: None,
        };

        assert_eq!(get_repo_rev(&r), Some("foo".into()));
    }

    #[test]
    fn get_detached() {
        let r = TestReference {
            shorthand: Some("HEAD"),
            short_id: Some("ea02629"),
            target: git2::Oid::from_str("ea026298c4856b690bc338e917235059fb1fe22a").ok(),
        };

        assert_eq!(get_repo_rev(&r), Some("ea02629".into()));
    }
}

pub fn branch_status(repo: &dyn Repo, name: &str, default: &str) -> R<BranchStatus> {
    let (ahead, behind) = repo
        .graph_ahead_behind(
            repo.head()
                .or_else(|e| Err(format!("{:?}", e)))?
                .target()
                .ok_or_else(|| "Failed to get target".to_owned())?,
            get_remote_ref(repo, name).or_else(|_| get_remote_ref(repo, default))?,
        )
        .or_else(|e| Err(format!("{:?}", e)))?;
    Ok(BranchStatus { ahead, behind })
}

fn get_remote_ref(repo: &dyn Repo, name: &str) -> R<git2::Oid> {
    repo.find_branch(name, git2::BranchType::Local)
        .or_else(|e| Err(format!("{:?}", e)))?
        .upstream()
        .or_else(|e| Err(format!("{:?}", e)))?
        .get()
        .target()
        .ok_or_else(|| "failed to get remote branch name".to_owned())
}

pub fn local_status(repo: &dyn Repo) -> LocalStatus {
    let is_staged = git2::Status::INDEX_NEW
        | git2::Status::INDEX_MODIFIED
        | git2::Status::INDEX_DELETED
        | git2::Status::INDEX_RENAMED
        | git2::Status::INDEX_TYPECHANGE;
    let is_modified = git2::Status::WT_MODIFIED
        | git2::Status::WT_DELETED
        | git2::Status::WT_RENAMED
        | git2::Status::WT_TYPECHANGE;

    let mut status = LocalStatus::new();
    if let Ok(statuses) = repo.statuses(Some(
        git2::StatusOptions::new()
            .include_ignored(false)
            .recurse_ignored_dirs(false)
            .include_untracked(true)
            .recurse_untracked_dirs(false),
    )) {
        for s in statuses.iter().map(|e| e.status()) {
            if s.is_wt_new() {
                status.untracked += 1;
            }
            if s.intersects(is_staged) {
                status.staged += 1;
            }
            if s.intersects(is_modified) {
                status.unstaged += 1;
            }
            if s.is_conflicted() {
                status.unmerged += 1;
            }
        }
    }
    status
}
