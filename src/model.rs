type RepoState = git2::RepositoryState;
pub type R<T> = Result<T, String>;

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
    pub fn add(&mut self, status: git2::Status) {
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

    fn status(s: git2::Status) -> LocalStatus {
        let mut actual = LocalStatus {
            ..Default::default()
        };
        actual.add(s);
        actual
    }

    #[test]
    fn untracked() {
        let expected = LocalStatus {
            untracked: 1,
            ..Default::default()
        };
        assert_eq!(status(git2::Status::WT_NEW), expected);
    }
    #[test]
    fn staged() {
        let expected = LocalStatus {
            staged: 1,
            ..Default::default()
        };
        assert_eq!(status(git2::Status::INDEX_NEW), expected);
        assert_eq!(status(git2::Status::INDEX_MODIFIED), expected);
        assert_eq!(status(git2::Status::INDEX_DELETED), expected);
        assert_eq!(status(git2::Status::INDEX_RENAMED), expected);
        assert_eq!(status(git2::Status::INDEX_TYPECHANGE), expected);
    }
    #[test]
    fn unstaged() {
        let expected = LocalStatus {
            unstaged: 1,
            ..Default::default()
        };
        assert_eq!(status(git2::Status::WT_MODIFIED), expected);
        assert_eq!(status(git2::Status::WT_DELETED), expected);
        assert_eq!(status(git2::Status::WT_RENAMED), expected);
        assert_eq!(status(git2::Status::WT_TYPECHANGE), expected);
    }
    #[test]
    fn conflict() {
        let expected = LocalStatus {
            unmerged: 1,
            ..Default::default()
        };
        assert_eq!(status(git2::Status::CONFLICTED), expected);
    }
    #[test]
    fn partial_stage() {
        let expected = LocalStatus {
            staged: 1,
            unstaged: 1,
            ..Default::default()
        };
        let mut s = git2::Status::WT_MODIFIED;
        s.insert(git2::Status::INDEX_MODIFIED);
        assert_eq!(status(s), expected);
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

pub fn repo_status(repo: &Repo) -> R<RepoStatus> {
    Ok(RepoStatus {
        branch: repo
            .head()
            .or_else(|e| Err(format!("{:?}", e)))
            .map(|r| get_repo_rev(&r))?,
        state: repo.state(),
    })
}

fn get_repo_rev(r: &Reference) -> Option<String> {
    match r.shorthand() {
        Some("HEAD") | None => r.target().map(|t| {
            let mut s = t.to_string();
            s.truncate(8);
            s.to_string()
        }),
        Some(b) => Some(b.to_owned()),
    }
}

pub trait Reference {
    fn shorthand(&self) -> Option<&str>;
    fn target(&self) -> Option<git2::Oid>;
}

impl<'repo> Reference for git2::Reference<'repo> {
    fn shorthand(&self) -> Option<&str> {
        self.shorthand()
    }
    fn target(&self) -> Option<git2::Oid> {
        self.target()
    }
}

#[cfg(test)]
mod repo_status {
    use super::*;
    use crate::test::Bencher;

    struct TestReference<'a> {
        shorthand: Option<&'a str>,
        target: Option<git2::Oid>,
    }

    impl<'a> Reference for TestReference<'a> {
        fn shorthand(&self) -> Option<&str> {
            self.shorthand
        }
        fn target(&self) -> Option<git2::Oid> {
            self.target
        }
    }

    #[test]
    fn get_shorthand() {
        let r = TestReference {
            shorthand: Some("foo"),
            target: None,
        };

        assert_eq!(get_repo_rev(&r), Some(String::from("foo")));
    }

    #[test]
    fn get_detached() {
        let r = TestReference {
            shorthand: Some("HEAD"),
            target: git2::Oid::from_str("ea026298c4856b690bc338e917235059fb1fe22a").ok(),
        };

        assert_eq!(get_repo_rev(&r), Some(String::from("ea026298")));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let r = git2::Repository::discover(".");
        b.iter(|| {
            r.as_ref()
                .or_else(|e| Err(format!("{:?}", e)))
                .and_then(|r| repo_status(r))
        });
    }
}

pub fn branch_status(repo: &Repo, name: &str, default: &str) -> R<BranchStatus> {
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

fn get_remote_ref(repo: &Repo, name: &str) -> R<git2::Oid> {
    repo.find_branch(name, git2::BranchType::Local)
        .or_else(|e| Err(format!("{:?}", e)))?
        .upstream()
        .or_else(|e| Err(format!("{:?}", e)))?
        .get()
        .target()
        .ok_or_else(|| "failed to get remote branch name".to_owned())
}

#[cfg(test)]
mod bench_branch_status {
    use super::*;
    use crate::test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let r = git2::Repository::discover(".");
        b.iter(|| {
            r.as_ref()
                .or_else(|e| Err(format!("{:?}", e)))
                .and_then(|r| branch_status(r, "master", "master"))
        });
    }
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
        status.add(s)
    }
    Ok(status)
}

#[cfg(test)]
mod bench_local_status {
    use super::*;
    use crate::test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let r = git2::Repository::discover(".");
        b.iter(|| {
            r.as_ref()
                .or_else(|e| Err(format!("{:?}", e)))
                .and_then(|r| local_status(r))
        });
    }
}
