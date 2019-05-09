use super::model::*;
use ansi_term::Color;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Prompt<'a> {
    pub repo: RepoStatus,
    // it only makes sense to have the branch status when the repo is OK
    pub branch: Option<BranchStatus>,
    pub local: Option<LocalStatus>,

    pub colors: Colors,
    pub branch_symbols: BranchSymbols<'a>,
    pub status_symbols: StatusSymbols<'a>,
}

impl<'a> Prompt<'a> {
    pub fn new(repo: &RepoStatus) -> Prompt<'a> {
        Prompt {
            repo: repo.clone(),
            branch: None,
            local: None,
            colors: NO_COLORS,
            branch_symbols: BranchSymbols {
                ahead: "↑",
                behind: "↓",
            },
            status_symbols: StatusSymbols {
                nothing: "✔",
                staged: "●",
                unmerged: "✖",
                unstaged: "✚",
                untracked: "…",
            },
        }
    }

    pub fn with_branch(&self, branch: Option<BranchStatus>) -> Prompt<'a> {
        let mut p = self.clone();
        p.branch = branch.clone();
        p
    }

    pub fn with_local(&self, local: Option<LocalStatus>) -> Prompt<'a> {
        let mut p = self.clone();
        p.local = local.clone();
        p
    }

    pub fn with_style(
        &self,
        c: &Colors,
        bs: &'a BranchSymbols,
        ss: &'a StatusSymbols,
    ) -> Prompt<'a> {
        let mut p = self.clone();
        p.colors = c.clone();
        p.branch_symbols = bs.clone();
        p.status_symbols = ss.clone();
        p
    }
}

impl<'a> Display for Prompt<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let state = format!(
            "{}",
            RepoStateView {
                model: self.repo.state,
                colors: &self.colors,
            }
        );
        let repo = format!(
            "{}",
            RepoStatusView {
                model: self.repo.clone(),
                colors: &self.colors,
            }
        );
        let branch = format!(
            "{}",
            BranchStatusView {
                model: self.branch.clone(),
                symbols: &self.branch_symbols,
                colors: &self.colors,
            }
        );
        let local = self
            .local
            .clone()
            .map(|status| LocalStatusView {
                model: status,
                symbols: &self.status_symbols,
                colors: &self.colors,
            })
            .map(|v| format!("{}", v))
            .unwrap_or_default();

        let mut r = String::new();
        for i in vec![state, repo, branch, local].iter() {
            if i != "" {
                r.push_str(i);
                r.push(' ');
            }
        }
        write!(f, "{}", r)
    }
}

#[cfg(test)]
mod print_tests {
    use super::*;

    #[test]
    fn prompt_is_respaced() {
        let p = Prompt::new(&RepoStatus {
            branch: Some(String::from("master")),
            state: git2::RepositoryState::Clean,
        })
        .with_branch(Some(BranchStatus {
            ahead: 1,
            behind: 4,
        }))
        .with_local(Some(LOCAL_CLEAN))
        .with_style(
            &NO_COLORS,
            &BranchSymbols {
                ahead: "↑",
                behind: "↓",
            },
            &StatusSymbols {
                nothing: "✓",
                staged: "s",
                unmerged: "m",
                unstaged: "u",
                untracked: ".",
            },
        );
        assert_eq!(p.to_string(), "master ↑1↓4 ✓ ");
    }

    #[test]
    fn prompt_is_trimmed() {
        let p = Prompt::new(&RepoStatus {
            branch: None,
            state: git2::RepositoryState::Clean,
        })
        .with_local(Some(LocalStatus {
            staged: 1,
            unmerged: 0,
            unstaged: 0,
            untracked: 3,
        }));
        let c = &NO_COLORS;
        let bs = BranchSymbols {
            ahead: "↑",
            behind: "↓",
        };
        let ss = StatusSymbols {
            nothing: "✓",
            staged: "s",
            unmerged: "m",
            unstaged: "u",
            untracked: ".",
        };
        assert_eq!(p.with_style(&c, &bs, &ss).to_string(), "s1. ");
    }
}

#[derive(Clone, Debug)]
pub struct Colors {
    pub ok: Option<Color>,
    pub high: Option<Color>,
    pub normal: Option<Color>,
}

pub const NO_COLORS: Colors = Colors {
    ok: None,
    high: None,
    normal: None,
};

#[derive(Clone, Debug)]
pub struct StatusSymbols<'a> {
    pub nothing: &'a str,
    pub staged: &'a str,
    pub unmerged: &'a str,
    pub unstaged: &'a str,
    pub untracked: &'a str,
}

#[derive(Clone, Debug)]
pub struct BranchSymbols<'a> {
    pub ahead: &'a str,
    pub behind: &'a str,
}

pub struct RepoStateView<'a> {
    pub model: git2::RepositoryState,
    pub colors: &'a Colors,
}

impl<'a> Display for RepoStateView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match &self.model {
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
        let s = View {
            text: s,
            color: self.colors.high,
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod repo_state_view {
    use super::*;

    #[test]
    fn empty() {
        let v = RepoStateView {
            model: git2::RepositoryState::Clean,
            colors: &NO_COLORS,
        };
        assert_eq!(format!("{}", v), "");
    }

    #[test]
    fn rebase() {
        let v = RepoStateView {
            model: git2::RepositoryState::Rebase,
            colors: &NO_COLORS,
        };
        assert_eq!(format!("{}", v), "rebase");
    }
}

pub struct RepoStatusView<'a> {
    pub model: RepoStatus,
    pub colors: &'a Colors,
}

impl<'a> Display for RepoStatusView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let b = self.model.branch.as_ref().map(|b| View {
            text: b,
            color: self.colors.normal,
        });
        if let Some(b) = b {
            write!(f, "{}", b)?;
        }
        Ok(())
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
            colors: &NO_COLORS,
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
            colors: &NO_COLORS,
        };
        assert_eq!(format!("{}", v), "master");
    }
}

pub struct BranchStatusView<'a> {
    pub model: Option<BranchStatus>,
    pub symbols: &'a BranchSymbols<'a>,
    pub colors: &'a Colors,
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
                write!(f, "{}{}", ahead, behind)
            })
            .unwrap_or(Ok(()))
    }
}

#[cfg(test)]
mod branch_status_view {
    use super::*;

    fn given(m: Option<BranchStatus>) -> String {
        let v = BranchStatusView {
            model: m,
            symbols: &BranchSymbols {
                ahead: "↑",
                behind: "↓",
            },
            colors: &super::NO_COLORS,
        };
        format!("{}", v)
    }

    fn given_some(ahead: usize, behind: usize) -> String {
        given(Some(BranchStatus { ahead, behind }))
    }

    #[test]
    fn is_empty() {
        assert_eq!(given(None), "");
        assert_eq!(given_some(0, 0), "");
    }

    #[test]
    fn ahead() {
        assert_eq!(given_some(6, 0), "↑6");
    }

    #[test]
    fn behind() {
        assert_eq!(given_some(1, 3), "↑1↓3");
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
    pub symbols: &'a StatusSymbols<'a>,
    pub colors: &'a Colors,
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
                color: self.colors.ok,
            };
            let untracked = View {
                text: if self.model.untracked == 0 {
                    ""
                } else {
                    self.symbols.untracked
                },
                color: None,
            };
            write!(f, "{}{}{}{}", unmerged, staged, unstaged, untracked)
        }
    }
}

#[cfg(test)]
mod local_status_view {
    use super::*;

    fn given(m: LocalStatus) -> String {
        let v = LocalStatusView {
            model: m,
            symbols: &StatusSymbols {
                nothing: "✔",
                staged: ".",
                unmerged: "x",
                unstaged: "+",
                untracked: "…",
            },
            colors: &NO_COLORS,
        };
        format!("{}", v)
    }

    #[test]
    fn clean() {
        let v = given(LocalStatus {
            staged: 0,
            unmerged: 0,
            unstaged: 0,
            untracked: 0,
        });
        assert_eq!(v, "✔");
    }

    #[test]
    fn zeroes_are_omitted() {
        let v = given(LocalStatus {
            staged: 1,
            unmerged: 0,
            unstaged: 0,
            untracked: 4,
        });
        assert_eq!(v, ".1…");
    }

    #[test]
    fn not_clean() {
        let v = given(LocalStatus {
            staged: 1,
            unmerged: 2,
            unstaged: 3,
            untracked: 4,
        });
        assert_eq!(v, "x2.1+3…");
    }
}

pub struct View<'a> {
    pub text: &'a str,
    pub color: Option<Color>,
}

impl<'a> Display for View<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.text {
            "" => Ok(()),
            t => match self.color {
                Some(c) => write!(f, "{}", c.paint(t)),
                None => write!(f, "{}", t),
            },
        }
    }
}

#[cfg(test)]
mod simple_view_tests {
    use super::View;
    use ansi_term::Color;

    fn given(text: &str, color: Option<Color>) -> String {
        format!("{}", View { text, color })
    }

    #[test]
    fn empty() {
        assert_eq!(given("", None), "");
        assert_eq!(given("", Some(Color::Red)), "");
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

    fn given(symbol: &str, n: usize, color: Option<Color>) -> String {
        format!("{}", StatView { symbol, n, color })
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
