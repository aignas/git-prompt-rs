use super::model::*;
use std::fmt::{self, Display, Formatter};

use ansi_term::Colour;

pub struct Colors {
    pub default: Colour,
    pub ok: Colour,
    pub high: Colour,
    pub normal: Colour,
    pub low: Colour,
}

pub const DEFAULT_COLORS: Colors = Colors {
    default: Colour::Fixed(7),
    ok: Colour::Fixed(2),
    high: Colour::Fixed(1),
    normal: Colour::Fixed(3),
    low: Colour::Fixed(4),
};

#[derive(Clone)]
pub struct StatusSymbols {
    pub nothing: char,
    pub staged: char,
    pub unmerged: char,
    pub unstaged: char,
    pub untracked: char,
}

const DEFAULT_STATUS_SYMBOLS: StatusSymbols = StatusSymbols {
    nothing: '✔',
    staged: '●',
    unmerged: '✖',
    unstaged: '✚',
    untracked: '…',
};

#[derive(Clone)]
pub struct BranchSymbols {
    pub ahead: char,
    pub behind: char,
}

const DEFAULT_BRANCH_SYMBOLS: BranchSymbols = BranchSymbols {
    ahead: '↑',
    behind: '↓',
};

pub struct PromptView {
    pub repo: RepoStatusView,
    pub branch: BranchStatusView,
    pub local: LocalStatusView,
}

impl PromptView {
    pub fn new(p: Prompt, _c: &Colors) -> PromptView {
        PromptView {
            repo: RepoStatusView {
                model: p.repo,
                colors: None,
            },
            branch: BranchStatusView {
                model: p.branch,
                symbols: DEFAULT_BRANCH_SYMBOLS,
                colors: None,
            },
            local: LocalStatusView {
                model: p.local,
                symbols: DEFAULT_STATUS_SYMBOLS,
                colors: None,
            },
        }
    }
}

impl Display for PromptView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{} {}", self.repo, self.branch, self.local)
    }
}

pub struct RepoStatusView {
    pub model: RepoStatus,
    pub colors: Option<Colors>,
}

impl Display for RepoStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match &self.model.state {
            git2::RepositoryState::Merge => "merge",
            git2::RepositoryState::Revert => "revert",
            git2::RepositoryState::RevertSequence => "revert…",
            git2::RepositoryState::CherryPick => "cherry-pick",
            git2::RepositoryState::CherryPickSequence => "cherry-pick…",
            git2::RepositoryState::Bisect => "bisect",
            git2::RepositoryState::Rebase => "rebase",
            git2::RepositoryState::RebaseInteractive => "rebase-i",
            git2::RepositoryState::RebaseMerge => "rebase-m",
            _ => "",
        };
        let maybe = if s == "" { None } else { Some(s) };

        match (&self.model.branch, &maybe) {
            (Some(a), Some(b)) => write!(f, "{} {}", a, b),
            (Some(a), None) => write!(f, "{}", a),
            (None, Some(b)) => write!(f, "{}", b),
            (None, None) => Ok(()),
        }
    }
}

#[cfg(test)]
mod repo_status_view {
    use super::*;

    #[test]
    fn nothing() {
        let v = RepoStatusView {
            model: RepoStatus {
                branch: None,
                state: git2::RepositoryState::Clean,
            },
        };
        assert_eq!(format!("{}", v), "");
    }

    #[test]
    fn branch_is_shown() {
        let v = RepoStatusView {
            model: RepoStatus {
                branch: Some("master".to_owned()),
                state: git2::RepositoryState::Clean,
            },
        };
        assert_eq!(format!("{}", v), "master");
    }

    #[test]
    fn rebase_is_shown() {
        let v = RepoStatusView {
            model: RepoStatus {
                branch: Some("master".to_owned()),
                state: git2::RepositoryState::Rebase,
            },
        };
        assert_eq!(format!("{}", v), "master rebase");
    }
}

pub struct BranchStatusView {
    pub model: Option<BranchStatus>,
    pub symbols: BranchSymbols,
    pub colors: Option<Colors>,
}

impl Display for BranchStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.model {
            Some(b) => {
                if b.ahead != 0 || b.behind != 0 {
                    write!(f, " ");
                }
                non_zero(self.symbols.ahead, b.ahead, f)?;
                non_zero(self.symbols.behind, b.behind, f)
            }
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod branch_status_view {
    use super::*;

    #[test]
    fn none() {
        let v = BranchStatusView {
            model: None,
            symbols: DEFAULT_BRANCH_SYMBOLS,
        };
        assert_eq!(format!("{}", v), "");
    }

    #[test]
    fn up_to_date() {
        let v = BranchStatusView {
            model: Some(BranchStatus {
                ahead: 0,
                behind: 0,
            }),
            symbols: DEFAULT_BRANCH_SYMBOLS,
        };
        assert_eq!(format!("{}", v), "");
    }

    #[test]
    fn ahead() {
        let v = BranchStatusView {
            model: Some(BranchStatus {
                ahead: 6,
                behind: 0,
            }),
            symbols: DEFAULT_BRANCH_SYMBOLS,
        };
        assert_eq!(format!("{}", v), " ↑6");
    }

    #[test]
    fn behind() {
        let v = BranchStatusView {
            model: Some(BranchStatus {
                ahead: 1,
                behind: 3,
            }),
            symbols: DEFAULT_BRANCH_SYMBOLS,
        };
        assert_eq!(format!("{}", v), " ↑1↓3");
    }
}

const LOCAL_CLEAN: LocalStatus = LocalStatus {
    staged: 0,
    unmerged: 0,
    unstaged: 0,
    untracked: 0,
};

pub struct LocalStatusView {
    pub model: LocalStatus,
    pub symbols: StatusSymbols,
    pub colors: Option<Colors>,
}

impl Display for LocalStatusView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if LOCAL_CLEAN == self.model {
            write!(f, "{}", self.symbols.nothing)
        } else {
            non_zero(self.symbols.unmerged, self.model.unmerged, f)?;
            non_zero(self.symbols.staged, self.model.staged, f)?;
            non_zero(self.symbols.unstaged, self.model.unstaged, f)?;
            if self.model.untracked != 0 {
                write!(f, "{}", self.symbols.untracked);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod local_status_view {
    use super::*;

    #[test]
    fn clean() {
        let v = LocalStatusView {
            model: LocalStatus {
                staged: 0,
                unmerged: 0,
                unstaged: 0,
                untracked: 0,
            },
            symbols: DEFAULT_STATUS_SYMBOLS,
        };
        assert_eq!(format!("{}", v), "✔");
    }

    #[test]
    fn zeroes_are_omitted() {
        let v = LocalStatusView {
            model: LocalStatus {
                staged: 1,
                unmerged: 0,
                unstaged: 0,
                untracked: 4,
            },
            symbols: DEFAULT_STATUS_SYMBOLS,
        };
        assert_eq!(format!("{}", v), "●1…");
    }

    #[test]
    fn not_clean() {
        let v = LocalStatusView {
            model: LocalStatus {
                staged: 1,
                unmerged: 2,
                unstaged: 3,
                untracked: 4,
            },
            symbols: DEFAULT_STATUS_SYMBOLS,
        };
        assert_eq!(format!("{}", v), "✖2●1✚3…");
    }
}

fn non_zero(prefix: char, number: usize, f: &mut Formatter) -> fmt::Result {
    if number != 0 {
        write!(f, "{}{}", prefix, number)
    } else {
        Ok(())
    }
}
