use super::model::*;
use std::fmt::{self, Display, Formatter};

use ansi_term::{ANSIString, Color};

#[derive(Clone)]
pub struct Colors {
    pub default: Color,
    pub ok: Color,
    pub high: Color,
    pub normal: Color,
    pub low: Color,
}

pub const DEFAULT_COLORS: Colors = Colors {
    default: Color::Fixed(7),
    ok: Color::Fixed(2),
    high: Color::Fixed(1),
    normal: Color::Fixed(3),
    low: Color::Fixed(4),
};

#[derive(Clone)]
pub struct StatusSymbols<'a> {
    pub nothing: &'a str,
    pub staged: &'a str,
    pub unmerged: &'a str,
    pub unstaged: &'a str,
    pub untracked: &'a str,
}

static DEFAULT_STATUS_SYMBOLS: StatusSymbols = StatusSymbols {
    nothing: "✔",
    staged: "●",
    unmerged: "✖",
    unstaged: "✚",
    untracked: "…",
};

#[derive(Clone)]
pub struct BranchSymbols<'a> {
    pub ahead: &'a str,
    pub behind: &'a str,
}

static DEFAULT_BRANCH_SYMBOLS: BranchSymbols = BranchSymbols {
    ahead: "↑",
    behind: "↓",
};

pub struct PromptView<'a> {
    pub repo: RepoStatusView,
    pub branch: BranchStatusView<'a>,
    pub local: LocalStatusView<'a>,
}

impl<'a> PromptView<'a> {
    pub fn new(p: Prompt, c: Option<Colors>) -> PromptView<'a> {
        PromptView {
            repo: RepoStatusView {
                model: p.repo,
                colors: c.clone(),
            },
            branch: BranchStatusView {
                model: p.branch,
                symbols: DEFAULT_BRANCH_SYMBOLS.clone(),
                colors: c.clone(),
            },
            local: LocalStatusView {
                model: p.local,
                symbols: DEFAULT_STATUS_SYMBOLS.clone(),
                colors: c,
            },
        }
    }
}

impl<'a> Display for PromptView<'a> {
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
            colors: None,
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
            colors: None,
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
            colors: None,
        };
        assert_eq!(format!("{}", v), "master rebase");
    }
}

pub struct BranchStatusView<'a> {
    pub model: Option<BranchStatus>,
    pub symbols: BranchSymbols<'a>,
    pub colors: Option<Colors>,
}

impl<'a> Display for BranchStatusView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.model {
            Some(b) => {
                if b.ahead == 0 && b.behind == 0 {
                    return Ok(());
                }
                let ahead = StatView {
                    symbol: self.symbols.ahead,
                    n: b.ahead,
                    color: self.colors.as_ref().map(|c| c.normal),
                };
                let behind = StatView {
                    symbol: self.symbols.behind,
                    n: b.behind,
                    color: self.colors.as_ref().map(|c| c.normal),
                };
                write!(f, " {}{}", ahead, behind)
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
            symbols: DEFAULT_BRANCH_SYMBOLS.clone(),
            colors: None,
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
            symbols: DEFAULT_BRANCH_SYMBOLS.clone(),
            colors: None,
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
            symbols: DEFAULT_BRANCH_SYMBOLS.clone(),
            colors: None,
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
            symbols: DEFAULT_BRANCH_SYMBOLS.clone(),
            colors: None,
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

pub struct LocalStatusView<'a> {
    pub model: LocalStatus,
    pub symbols: StatusSymbols<'a>,
    pub colors: Option<Colors>,
}

impl<'a> Display for LocalStatusView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if LOCAL_CLEAN == self.model {
            match &self.colors {
                Some(c) => write!(f, "{}", c.ok.paint(format!("{}", self.symbols.nothing))),
                None => write!(f, "{}", self.symbols.nothing),
            }
        } else {
            let unmerged = StatView {
                symbol: self.symbols.unmerged,
                n: self.model.unmerged,
                color: self.colors.as_ref().map(|c| c.high),
            };
            let unstaged = StatView {
                symbol: self.symbols.unstaged,
                n: self.model.unstaged,
                color: self.colors.as_ref().map(|c| c.high),
            };
            let staged = StatView {
                symbol: self.symbols.staged,
                n: self.model.staged,
                color: self.colors.as_ref().map(|c| c.high),
            };
            let untracked = if self.model.untracked == 0 {
                ""
            } else {
                self.symbols.untracked
            };
            write!(f, "{}{}{}{}", unmerged, staged, unstaged, untracked)
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
            symbols: DEFAULT_STATUS_SYMBOLS.clone(),
            colors: None,
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
            symbols: DEFAULT_STATUS_SYMBOLS.clone(),
            colors: None,
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
            symbols: DEFAULT_STATUS_SYMBOLS.clone(),
            colors: None,
        };
        assert_eq!(format!("{}", v), "✖2●1✚3…");
    }
}

pub struct StatView<'a> {
    pub symbol: &'a str,
    pub n: usize,
    pub color: Option<Color>,
}

impl<'a> Display for StatView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.n {
            0 => Ok(()),
            n => match self.color {
                Some(c) => write!(f, "{}{}", c.paint(self.symbol), n),
                None => write!(f, "{}{}", self.symbol, n),
            },
        }
    }
}

fn non_zero(prefix: ANSIString, number: usize, f: &mut Formatter) -> fmt::Result {
    match number {
        0 => Ok(()),
        n => write!(f, "{}{}", prefix, n),
    }
}
