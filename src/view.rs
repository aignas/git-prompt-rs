use super::model::*;
use std::fmt::{self, Display, Formatter};

use ansi_term::Color;

#[derive(Clone)]
pub struct Colors {
    pub default: Option<Color>,
    pub ok: Option<Color>,
    pub high: Option<Color>,
    pub normal: Option<Color>,
    pub low: Option<Color>,
}

#[cfg(test)]
pub const NO_COLORS: Colors = Colors {
    default: None,
    ok: None,
    high: None,
    normal: None,
    low: None,
};

pub const DEFAULT_COLORS: Colors = Colors {
    default: Some(Color::Fixed(7)),
    ok: Some(Color::Green),
    high: Some(Color::Red),
    normal: Some(Color::Yellow),
    low: Some(Color::Fixed(252)),
};

static DEFAULT_STATUS_SYMBOLS: StatusSymbols = StatusSymbols {
    nothing: "✔",
    staged: "●",
    unmerged: "✖",
    unstaged: "✚",
    untracked: "…",
};

static DEFAULT_BRANCH_SYMBOLS: BranchSymbols = BranchSymbols {
    ahead: "↑",
    behind: "↓",
};

#[derive(Clone)]
pub struct StatusSymbols<'a> {
    pub nothing: &'a str,
    pub staged: &'a str,
    pub unmerged: &'a str,
    pub unstaged: &'a str,
    pub untracked: &'a str,
}

#[derive(Clone)]
pub struct BranchSymbols<'a> {
    pub ahead: &'a str,
    pub behind: &'a str,
}

pub struct PromptView<'a> {
    pub repo: RepoStatusView,
    pub branch: BranchStatusView<'a>,
    pub local: LocalStatusView<'a>,
}

impl<'a> PromptView<'a> {
    pub fn new(p: Prompt, c: Colors) -> PromptView<'a> {
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
    pub colors: Colors,
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
        let s = if s == "" { None } else { Some(s) };
        let s = s.map(|text| View {
            text: text,
            color: self.colors.high,
        });
        let b = self.model.branch.as_ref().map(|b| View {
            text: b,
            color: self.colors.normal,
        });
        match (b, s) {
            (None, None) => Ok(()),
            (None, Some(a)) => write!(f, "{}", a),
            (Some(a), None) => write!(f, "{}", a),
            (Some(a), Some(b)) => write!(f, "{} {}", a, b),
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
        };
        assert_eq!(format!("{}", v), "master rebase");
    }
}

pub struct BranchStatusView<'a> {
    pub model: Option<BranchStatus>,
    pub symbols: BranchSymbols<'a>,
    pub colors: Colors,
}

impl<'a> Display for BranchStatusView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.model
            .as_ref()
            .map(|b| {
                if b.ahead == 0 && b.behind == 0 {
                    return Ok(());
                }
                let ahead = StatView {
                    symbol: self.symbols.ahead,
                    n: b.ahead,
                    color: self.colors.normal,
                };
                let behind = StatView {
                    symbol: self.symbols.behind,
                    n: b.behind,
                    color: self.colors.normal,
                };
                write!(f, " {}{}", ahead, behind)
            })
            .unwrap_or(Ok(()))
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
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
    pub colors: Colors,
}

impl<'a> Display for LocalStatusView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if LOCAL_CLEAN == self.model {
            let v = View {
                text: self.symbols.nothing,
                color: self.colors.ok,
            };
            write!(f, "{}", v)
        } else {
            let unmerged = StatView {
                symbol: self.symbols.unmerged,
                n: self.model.unmerged,
                color: self.colors.high,
            };
            let unstaged = StatView {
                symbol: self.symbols.unstaged,
                n: self.model.unstaged,
                color: self.colors.normal,
            };
            let staged = StatView {
                symbol: self.symbols.staged,
                n: self.model.staged,
                color: self.colors.normal,
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
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
            colors: NO_COLORS.clone(),
        };
        assert_eq!(format!("{}", v), "✖2●1✚3…");
    }
}

pub struct View<'a> {
    pub text: &'a str,
    pub color: Option<Color>,
}

impl<'a> Display for View<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.color {
            Some(c) => write!(f, "{}", c.paint(self.text)),
            None => write!(f, "{}", self.text),
        }
    }
}

#[cfg(test)]
mod simple_view_tests {
    use super::View;
    use ansi_term::Color;

    fn given(text: &str, c: Option<Color>) -> String {
        let v = View {
            text: text,
            color: c,
        };
        format!("{}", v)
    }

    #[test]
    fn correct_text() {
        assert_eq!(given("foo", None), "foo");
        assert_eq!(given("bar", None), "bar");
    }

    #[test]
    fn correct_color() {
        assert_eq!(
            given("foo", Some(Color::Fixed(1))),
            "\u{1b}[38;5;1mfoo\u{1b}[0m"
        );
        assert_eq!(
            given("foo", Some(Color::Fixed(2))),
            "\u{1b}[38;5;2mfoo\u{1b}[0m"
        );
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

#[cfg(test)]
mod stat_view_tests {
    use super::StatView;
    use ansi_term::Color;

    fn given(prefix: &str, n: usize, c: Option<Color>) -> String {
        let v = StatView {
            symbol: prefix,
            n: n,
            color: c,
        };
        format!("{}", v)
    }

    #[test]
    fn no_text() {
        assert_eq!(given("foo", 0, None), "");
        assert_eq!(given("bar", 0, None), "");
        assert_eq!(given("foo", 0, Some(Color::Red)), "");
    }

    #[test]
    fn text() {
        assert_eq!(given("foo", 1, None), "foo1");
        assert_eq!(given("bar", 1, None), "bar1");
    }

    #[test]
    fn number() {
        assert_eq!(given("foo", 1, None), "foo1");
        assert_eq!(given("foo", 2, None), "foo2");
        assert_eq!(given("foo", 3, None), "foo3");
    }

    #[test]
    fn color() {
        let colors = vec![1, 2, 3];
        for c in colors {
            assert_eq!(
                given("foo", 1, Some(Color::Fixed(c))),
                format!("{}1", Color::Fixed(c).paint("foo"))
            );
        }
    }
}
